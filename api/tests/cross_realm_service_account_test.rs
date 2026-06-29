/// Cross-realm admin via role-assigned service account (policy path, no custom JWT claims).
///
/// Run with: cargo test -p ferriskey-api --test cross_realm_service_account_test -- --ignored
#[cfg(test)]
mod tests {
    use std::sync::Arc;

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
        std::env::var(key).unwrap_or_else(|_| default.to_string())
    }

    fn env_u16_or(key: &str, default: u16) -> u16 {
        std::env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    struct TestContext {
        server: TestServer,
    }

    async fn setup() -> TestContext {
        let db_host = env_or("DATABASE_HOST", "localhost");
        let db_port = env_u16_or("DATABASE_PORT", 5432);
        let db_name = env_or("DATABASE_NAME", "ferriskey");
        let db_user = env_or("DATABASE_USER", "ferriskey");
        let db_password = env_or("DATABASE_PASSWORD", "ferriskey");

        let schema = format!("cross_realm_test_{}", Uuid::new_v4().simple());

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

    async fn admin_token(ctx: &TestContext) -> String {
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
        assert_eq!(response.status_code(), 200, "admin token failed");
        response.json::<Value>()["access_token"]
            .as_str()
            .unwrap()
            .to_string()
    }

    fn decode_jwt_payload(token: &str) -> Value {
        let payload_b64 = token.split('.').nth(1).expect("jwt payload segment");
        let bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(payload_b64)
            .expect("payload is base64url");
        serde_json::from_slice(&bytes).expect("payload is json")
    }

    #[tokio::test]
    #[ignore]
    async fn service_account_lists_tenant_clients_via_cross_realm_role() {
        let ctx = setup().await;
        let admin = admin_token(&ctx).await;

        let tenant = format!("tenant-{}", Uuid::new_v4().simple());
        let create_realm = ctx
            .server
            .post("/realms")
            .add_header("Authorization", auth_header(&admin))
            .json(&json!({ "name": tenant }))
            .await;
        assert_eq!(
            create_realm.status_code(),
            201,
            "create tenant failed: {}",
            create_realm.text()
        );

        let role_name = format!("{tenant}-realm");
        let roles: Value = ctx
            .server
            .get("/realms/master/roles")
            .add_header("Authorization", auth_header(&admin))
            .await
            .json();
        let cross_role_id = roles["data"]
            .as_array()
            .expect("roles array")
            .iter()
            .find(|r| r["name"].as_str() == Some(role_name.as_str()))
            .and_then(|r| r["id"].as_str())
            .expect("cross-realm role not found");

        let patch = ctx
            .server
            .patch(&format!(
                "/realms/master/roles/{cross_role_id}/permissions"
            ))
            .add_header("Authorization", auth_header(&admin))
            .json(&json!({ "permissions": ["manage_realm", "view_clients"] }))
            .await;
        assert_eq!(
            patch.status_code(),
            200,
            "patch role permissions failed: {}",
            patch.text()
        );

        let sa_client = ctx
            .server
            .post("/realms/master/clients")
            .add_header("Authorization", auth_header(&admin))
            .json(&json!({
                "client_id": "automation-sa",
                "name": "Automation SA",
                "client_type": "confidential",
                "protocol": "openid-connect",
                "public_client": false,
                "service_account_enabled": true,
                "direct_access_grants_enabled": false,
                "enabled": true,
            }))
            .await;
        assert_eq!(
            sa_client.status_code(),
            201,
            "create automation client failed: {}",
            sa_client.text()
        );
        let sa_secret = sa_client.json::<Value>()["secret"]
            .as_str()
            .expect("client secret on create")
            .to_string();

        let users: Value = ctx
            .server
            .get("/realms/master/users")
            .add_header("Authorization", auth_header(&admin))
            .await
            .json();
        let sa_user_id = users["data"]
            .as_array()
            .expect("users array")
            .iter()
            .find(|u| u["username"].as_str() == Some("service-account-automation-sa"))
            .and_then(|u| u["id"].as_str())
            .expect("service account user not found");

        let assign = ctx
            .server
            .post(&format!(
                "/realms/master/users/{sa_user_id}/roles/{cross_role_id}"
            ))
            .add_header("Authorization", auth_header(&admin))
            .await;
        assert_eq!(
            assign.status_code(),
            200,
            "assign cross-realm role failed: {}",
            assign.text()
        );

        let token_resp = ctx
            .server
            .post("/realms/master/protocol/openid-connect/token")
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", "automation-sa"),
                ("client_secret", sa_secret.as_str()),
            ])
            .await;
        assert_eq!(
            token_resp.status_code(),
            200,
            "client_credentials failed: {}",
            token_resp.text()
        );
        let access_token = token_resp.json::<Value>()["access_token"]
            .as_str()
            .unwrap()
            .to_string();

        let claims = decode_jwt_payload(&access_token);
        assert!(
            claims.get("platform_operator").is_none(),
            "token must not carry platform_operator"
        );
        assert!(
            claims.get("managed_realms").is_none(),
            "token must not carry managed_realms"
        );

        let list = ctx
            .server
            .get(&format!("/realms/{tenant}/clients"))
            .add_header("Authorization", auth_header(&access_token))
            .await;
        assert_eq!(
            list.status_code(),
            200,
            "cross-realm client list failed: {}",
            list.text()
        );
    }
}
