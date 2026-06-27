/// Integration tests for the tenant bootstrap / import API (FK-2 / FK-5).
///
/// Run with:
///   cargo test -p ferriskey-api -- --ignored
#[cfg(test)]
mod tests {
    use std::{env, sync::Arc};

    use axum::http::HeaderValue;
    use axum_test::TestServer;
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

    async fn setup() -> TestContext {
        let db_host = env_or("DATABASE_HOST", "localhost");
        let db_port = env_u16_or("DATABASE_PORT", 5432);
        let db_name = env_or("DATABASE_NAME", "ferriskey");
        let db_user = env_or("DATABASE_USER", "ferriskey");
        let db_password = env_or("DATABASE_PASSWORD", "ferriskey");

        let schema = format!("bootstrap_test_{}", Uuid::new_v4().simple());

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

    #[tokio::test]
    #[ignore]
    async fn bootstrap_is_idempotent_and_seeds_roles() {
        let ctx = setup().await;
        let admin_token = get_admin_token(&ctx).await;
        let tenant = format!("tenant-{}", Uuid::new_v4().simple());

        let first = ctx
            .server
            .post(&format!("/realms/{}/bootstrap", tenant))
            .add_header("Authorization", auth_header(&admin_token))
            .json(&json!({
                "include_tenant_m2m": true,
                "bootstrap_admin": {
                    "username": "superadmin",
                    "password": "superadmin",
                },
            }))
            .await;

        assert_eq!(
            first.status_code(),
            200,
            "first bootstrap failed: {}",
            first.text()
        );
        let report: Value = first.json();
        assert_eq!(report["realm_created"], Value::Bool(true));
        assert_eq!(report["admin_user_created"], Value::Bool(true));
        let roles = report["roles_created"].as_array().unwrap();
        let role_names: Vec<&str> = roles.iter().filter_map(|r| r.as_str()).collect();
        for expected in ["superadmin", "admin", "staff"] {
            assert!(
                role_names.contains(&expected),
                "missing seeded role {expected}"
            );
        }

        // Second run is idempotent: realm already exists, no new admin/realm.
        let second = ctx
            .server
            .post(&format!("/realms/{}/bootstrap", tenant))
            .add_header("Authorization", auth_header(&admin_token))
            .json(&json!({
                "include_tenant_m2m": true,
                "bootstrap_admin": {
                    "username": "superadmin",
                    "password": "superadmin",
                },
            }))
            .await;

        assert_eq!(
            second.status_code(),
            200,
            "second bootstrap failed: {}",
            second.text()
        );
        let report2: Value = second.json();
        assert_eq!(
            report2["realm_created"],
            Value::Bool(false),
            "realm should not be re-created on second bootstrap"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn import_resolves_default_template() {
        let ctx = setup().await;
        let admin_token = get_admin_token(&ctx).await;
        let tenant = format!("tenant-{}", Uuid::new_v4().simple());

        let response = ctx
            .server
            .post(&format!("/realms/{}/import", tenant))
            .add_header("Authorization", auth_header(&admin_token))
            .json(&json!({
                "template": "tenant-default",
                "bootstrap_admin": {
                    "username": "superadmin",
                    "password": "superadmin",
                },
            }))
            .await;

        assert_eq!(
            response.status_code(),
            200,
            "import failed: {}",
            response.text()
        );
        let report: Value = response.json();
        assert_eq!(report["realm_created"], Value::Bool(true));
        assert_eq!(report["realm_name"], json!(tenant));
    }

    #[tokio::test]
    #[ignore]
    async fn bootstrap_backfills_platform_operator_on_existing_realm() {
        let ctx = setup().await;
        let admin_token = get_admin_token(&ctx).await;
        let tenant = format!("legacy-{}", Uuid::new_v4().simple());

        // Create realm only (no bootstrap clients).
        let create = ctx
            .server
            .post("/realms")
            .add_header("Authorization", auth_header(&admin_token))
            .json(&json!({ "realm_name": tenant }))
            .await;
        assert_eq!(create.status_code(), 201, "create realm: {}", create.text());

        // Bootstrap/import on existing realm must succeed and backfill operator wiring.
        let bootstrap = ctx
            .server
            .post(&format!("/realms/{}/import", tenant))
            .add_header("Authorization", auth_header(&admin_token))
            .json(&json!({
                "template": "tenant-default",
                "bootstrap_admin": {
                    "username": "superadmin",
                    "password": "superadmin",
                },
            }))
            .await;
        assert_eq!(
            bootstrap.status_code(),
            200,
            "import on existing realm failed: {}",
            bootstrap.text()
        );
        let report: Value = bootstrap.json();
        assert_eq!(report["realm_created"], Value::Bool(false));
        assert_eq!(report["web_client_created"], Value::Bool(true));
    }

    #[tokio::test]
    #[ignore]
    async fn import_rejects_unknown_template() {
        let ctx = setup().await;
        let admin_token = get_admin_token(&ctx).await;
        let tenant = format!("tenant-{}", Uuid::new_v4().simple());

        let response = ctx
            .server
            .post(&format!("/realms/{}/import", tenant))
            .add_header("Authorization", auth_header(&admin_token))
            .json(&json!({ "template": "does-not-exist" }))
            .await;

        assert_eq!(
            response.status_code(),
            400,
            "unknown template should be rejected, got: {}",
            response.text()
        );
    }
}
