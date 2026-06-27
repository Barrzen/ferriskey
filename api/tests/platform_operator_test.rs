/// Integration tests for the platform operator (FK-1 / FK-5).
///
/// The master-realm `auth-svc-m2m` service account is granted cross-realm admin
/// rights on realm creation and its client_credentials access token carries the
/// `platform_operator` / `managed_realms` claims.
///
/// Run with:
///   cargo test -p ferriskey-api -- --ignored
#[cfg(test)]
mod tests {
    use std::{env, sync::Arc};

    use axum::http::HeaderValue;
    use axum_test::TestServer;
    use base64::{Engine, engine::general_purpose};
    use ferriskey_api::{
        application::http::server::{app_state::AppState, http_server::router},
        args::Args,
    };
    use ferriskey_core::{
        application::create_service,
        domain::common::{
            DatabaseConfig, FerriskeyConfig, entities::StartupConfig, ports::CoreService,
        },
    };
    use serde_json::{Value, json};
    use sqlx::Executor;
    use uuid::Uuid;

    fn env_or(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    fn env_u16_or(key: &str, default: u16) -> u16 {
        env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    struct TestContext {
        server: TestServer,
    }

    /// Boots a server whose master realm is literally `master` so the platform
    /// operator claim path (`realm_name == "master"`) is exercised.
    async fn setup() -> TestContext {
        let db_host = env_or("DATABASE_HOST", "localhost");
        let db_port = env_u16_or("DATABASE_PORT", 5432);
        let db_name = env_or("DATABASE_NAME", "ferriskey");
        let db_user = env_or("DATABASE_USER", "ferriskey");
        let db_password = env_or("DATABASE_PASSWORD", "ferriskey");

        let schema = format!("platform_op_test_{}", Uuid::new_v4().simple());

        let admin_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        );

        let admin_pool = sqlx::PgPool::connect(&admin_url)
            .await
            .expect("connect admin pool");

        admin_pool
            .execute(sqlx::query(&format!(
                "CREATE SCHEMA IF NOT EXISTS \"{}\"",
                schema
            )))
            .await
            .expect("create schema");

        let schema_url = format!(
            "postgres://{}:{}@{}:{}/{}?options=-c search_path={}",
            db_user,
            db_password,
            db_host,
            db_port,
            db_name,
            urlencoding::encode(&schema)
        );

        let pool = sqlx::PgPool::connect(&schema_url)
            .await
            .expect("connect schema pool");

        sqlx::migrate!("../core/migrations")
            .run(&pool)
            .await
            .expect("run migrations");

        let service = create_service(FerriskeyConfig {
            database: DatabaseConfig {
                host: db_host,
                port: db_port,
                username: db_user,
                password: db_password,
                name: db_name,
                schema: schema.clone(),
            },
        })
        .await
        .expect("create service");

        service
            .initialize_application(StartupConfig {
                master_realm_name: "master".to_string(),
                admin_username: "admin".to_string(),
                admin_password: "admin".to_string(),
                admin_email: "admin@test.local".to_string(),
                default_client_id: "admin-cli".to_string(),
            })
            .await
            .expect("initialize application");

        let args = Arc::new(Args::default());
        let state = AppState::new(args, service);
        let app = router(state).expect("build router");
        let server = TestServer::new(app).expect("create test server");

        TestContext { server }
    }

    fn auth_header(token: &str) -> HeaderValue {
        format!("Bearer {}", token).parse().unwrap()
    }

    async fn get_admin_token(ctx: &TestContext) -> String {
        let response = ctx
            .server
            .post("/realms/master/protocol/openid-connect/token")
            .form(&[
                ("grant_type", "password"),
                ("client_id", "admin-cli"),
                ("username", "admin"),
                ("password", "admin"),
            ])
            .await;

        assert_eq!(response.status_code(), 200, "admin token request failed");
        let body: Value = response.json();
        body["access_token"].as_str().unwrap().to_string()
    }

    /// Decode the (unverified) JWT payload segment into a JSON value.
    fn decode_jwt_payload(token: &str) -> Value {
        let payload_b64 = token.split('.').nth(1).expect("jwt has a payload segment");
        let bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(payload_b64)
            .expect("payload is base64url");
        serde_json::from_slice(&bytes).expect("payload is json")
    }

    async fn create_platform_m2m(ctx: &TestContext, admin_token: &str) -> String {
        let response = ctx
            .server
            .post("/realms/master/clients")
            .add_header("Authorization", auth_header(admin_token))
            .json(&json!({
                "client_id": "auth-svc-m2m",
                "name": "Platform M2M",
                "client_type": "confidential",
                "protocol": "openid-connect",
                "public_client": false,
                "service_account_enabled": true,
                "direct_access_grants_enabled": false,
                "enabled": true,
            }))
            .await;

        assert_eq!(
            response.status_code(),
            201,
            "auth-svc-m2m creation failed: {}",
            response.text()
        );
        let body: Value = response.json();
        body["secret"].as_str().unwrap().to_string()
    }

    #[tokio::test]
    #[ignore]
    async fn platform_operator_token_carries_claims() {
        let ctx = setup().await;
        let admin_token = get_admin_token(&ctx).await;
        let secret = create_platform_m2m(&ctx, &admin_token).await;

        let response = ctx
            .server
            .post("/realms/master/protocol/openid-connect/token")
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", "auth-svc-m2m"),
                ("client_secret", secret.as_str()),
            ])
            .await;

        assert_eq!(
            response.status_code(),
            200,
            "m2m client_credentials failed: {}",
            response.text()
        );
        let body: Value = response.json();
        let access_token = body["access_token"].as_str().unwrap();
        let claims = decode_jwt_payload(access_token);

        assert_eq!(
            claims["platform_operator"],
            Value::Bool(true),
            "platform_operator claim missing"
        );
        assert_eq!(claims["managed_realms"], json!(["*"]));
    }

    #[tokio::test]
    #[ignore]
    async fn platform_operator_can_list_tenant_clients() {
        let ctx = setup().await;
        let admin_token = get_admin_token(&ctx).await;
        let secret = create_platform_m2m(&ctx, &admin_token).await;

        // Create a tenant realm; FK-1 assigns the cross-realm role to the M2M
        // service account during creation.
        let tenant = format!("tenant-{}", Uuid::new_v4().simple());
        let create = ctx
            .server
            .post("/realms")
            .add_header("Authorization", auth_header(&admin_token))
            .json(&json!({ "name": tenant }))
            .await;
        assert_eq!(
            create.status_code(),
            201,
            "tenant realm creation failed: {}",
            create.text()
        );

        // M2M token (issued on master) should be able to read tenant clients.
        let m2m = ctx
            .server
            .post("/realms/master/protocol/openid-connect/token")
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", "auth-svc-m2m"),
                ("client_secret", secret.as_str()),
            ])
            .await;
        let m2m_token = m2m.json::<Value>()["access_token"]
            .as_str()
            .unwrap()
            .to_string();

        let list = ctx
            .server
            .get(&format!("/realms/{}/clients", tenant))
            .add_header("Authorization", auth_header(&m2m_token))
            .await;

        assert_eq!(
            list.status_code(),
            200,
            "platform operator could not list tenant clients: {}",
            list.text()
        );
    }
}
