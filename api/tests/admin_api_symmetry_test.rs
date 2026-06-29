/// Integration tests for F1 admin API symmetry: secret redaction, role permissions GET, idempotent redirects.
///
/// Run with: cargo test -p ferriskey-api --test admin_api_symmetry_test -- --ignored
#[cfg(test)]
mod tests {
    use std::sync::Arc;

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

        let schema = format!("admin_symmetry_test_{}", Uuid::new_v4().simple());

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
        assert_eq!(response.status_code(), 200);
        response.json::<Value>()["access_token"]
            .as_str()
            .unwrap()
            .to_string()
    }

    #[tokio::test]
    #[ignore]
    async fn client_secret_redacted_on_list_exposed_on_secret_endpoint() {
        let ctx = setup().await;
        let admin = admin_token(&ctx).await;

        let create = ctx
            .server
            .post("/realms/master/clients")
            .add_header("Authorization", auth_header(&admin))
            .json(&json!({
                "client_id": "symmetry-confidential",
                "name": "Symmetry Confidential",
                "client_type": "confidential",
                "protocol": "openid-connect",
                "public_client": false,
                "service_account_enabled": false,
                "enabled": true,
            }))
            .await;
        assert_eq!(create.status_code(), 201);
        let created: Value = create.json();
        let client_uuid = created["id"].as_str().unwrap();
        let created_secret = created["secret"].as_str().expect("secret on create");

        let list: Value = ctx
            .server
            .get("/realms/master/clients")
            .add_header("Authorization", auth_header(&admin))
            .await
            .json();
        let listed = list["data"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"].as_str() == Some(client_uuid))
            .expect("client in list");
        assert!(
            listed["secret"].is_null(),
            "list must redact confidential secret"
        );

        let secret_resp: Value = ctx
            .server
            .get(&format!("/realms/master/clients/{client_uuid}/secret"))
            .add_header("Authorization", auth_header(&admin))
            .await
            .json();
        assert_eq!(
            secret_resp["secret"].as_str(),
            Some(created_secret),
            "secret endpoint must return the confidential secret"
        );
    }

    #[tokio::test]
    #[ignore]
    async fn role_permissions_get_matches_patch() {
        let ctx = setup().await;
        let admin = admin_token(&ctx).await;

        let create = ctx
            .server
            .post("/realms/master/roles")
            .add_header("Authorization", auth_header(&admin))
            .json(&json!({
                "name": format!("symmetry-role-{}", Uuid::new_v4().simple()),
                "permissions": ["view_clients"],
            }))
            .await;
        assert_eq!(create.status_code(), 201);
        let role_id = create.json::<Value>()["data"]["id"]
            .as_str()
            .unwrap()
            .to_string();

        let patch = ctx
            .server
            .patch(&format!("/realms/master/roles/{role_id}/permissions"))
            .add_header("Authorization", auth_header(&admin))
            .json(&json!({ "permissions": ["view_clients", "manage_users"] }))
            .await;
        assert_eq!(patch.status_code(), 200, "{}", patch.text());

        let get: Value = ctx
            .server
            .get(&format!("/realms/master/roles/{role_id}/permissions"))
            .add_header("Authorization", auth_header(&admin))
            .await
            .json();
        let perms: Vec<&str> = get["data"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|p| p.as_str())
            .collect();
        assert!(perms.contains(&"view_clients"));
        assert!(perms.contains(&"manage_users"));
    }

    #[tokio::test]
    #[ignore]
    async fn create_redirect_uri_is_idempotent() {
        let ctx = setup().await;
        let admin = admin_token(&ctx).await;

        let client = ctx
            .server
            .post("/realms/master/clients")
            .add_header("Authorization", auth_header(&admin))
            .json(&json!({
                "client_id": "symmetry-redirects",
                "name": "Symmetry Redirects",
                "client_type": "public",
                "protocol": "openid-connect",
                "public_client": true,
                "enabled": true,
            }))
            .await;
        assert_eq!(client.status_code(), 201);
        let client_body: Value = client.json();
        let client_uuid = client_body["id"].as_str().unwrap();

        let redirect_value = "https://example.test/callback";
        let body = json!({ "value": redirect_value, "enabled": true });

        let first = ctx
            .server
            .post(&format!("/realms/master/clients/{client_uuid}/redirects"))
            .add_header("Authorization", auth_header(&admin))
            .json(&body)
            .await;
        assert_eq!(first.status_code(), 201, "{}", first.text());
        let first_id = first.json::<Value>()["id"].as_str().unwrap().to_string();

        let second = ctx
            .server
            .post(&format!("/realms/master/clients/{client_uuid}/redirects"))
            .add_header("Authorization", auth_header(&admin))
            .json(&body)
            .await;
        assert_eq!(second.status_code(), 201, "{}", second.text());
        let second_body: Value = second.json();
        let second_id = second_body["id"].as_str().unwrap();
        assert_eq!(
            second_id, first_id,
            "idempotent redirect POST must return existing URI"
        );

        let list: Value = ctx
            .server
            .get(&format!("/realms/master/clients/{client_uuid}/redirects"))
            .add_header("Authorization", auth_header(&admin))
            .await
            .json();
        let uris = list.as_array().expect("redirect URI list is a JSON array");
        let matching: Vec<_> = uris
            .iter()
            .filter(|u| u["value"].as_str() == Some(redirect_value))
            .collect();
        assert_eq!(matching.len(), 1, "only one redirect URI row expected");
    }
}
