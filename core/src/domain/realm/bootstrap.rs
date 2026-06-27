//! Idempotent tenant-realm bootstrap.
//!
//! Mirrors auth-api's `scripts/create-realm.sh`: provisions the reserved
//! service clients (`auth-svc-web`, `auth-svc-login-ui`, and optionally a
//! tenant-local `auth-svc-m2m`), the product roles (`superadmin`, `admin`,
//! `staff`), and a bootstrap admin user. Every step is safe to re-run.

use uuid::Uuid;

use crate::domain::{
    abyss::identity_provider::IdentityProviderRepository,
    authentication::value_objects::Identity,
    client::{
        entities::{Client, ClientType},
        ports::{ClientRepository, RedirectUriRepository},
        value_objects::{CreateClientRequest, UpdateClientRequest},
    },
    common::{entities::app_errors::CoreError, generate_random_string},
    credential::ports::CredentialRepository,
    crypto::HasherRepository,
    realm::{
        entities::RealmId,
        ports::{
            BootstrapRealmInput, BootstrapRealmReport, CreateRealmInput, RealmRepository,
            RealmService,
        },
        services::RealmServiceImpl,
    },
    role::{ports::RoleRepository, value_objects::CreateRoleRequest},
    user::{
        ports::{UserRepository, UserRoleRepository},
        value_objects::CreateUserRequest,
    },
    webhook::ports::WebhookRepository,
};
use ferriskey_aegis::ports::{
    ClientScopeMappingRepository, ClientScopeRepository, ProtocolMapperRepository,
};

/// Reserved confidential web/BFF client. Must match `auth-config`'s `reserved.rs`.
pub const AUTH_SVC_WEB_CLIENT_ID: &str = "auth-svc-web";
/// Reserved public login-ui SPA client.
pub const AUTH_SVC_LOGIN_UI_CLIENT_ID: &str = "auth-svc-login-ui";
/// Product roles seeded into every tenant realm.
pub const BOOTSTRAP_ROLES: [&str; 3] = ["superadmin", "admin", "staff"];

impl<R, U, C, UR, RO, W, I, CS, PM, CSM, RU, CR, H>
    RealmServiceImpl<R, U, C, UR, RO, W, I, CS, PM, CSM, RU, CR, H>
where
    R: RealmRepository,
    U: UserRepository,
    C: ClientRepository,
    UR: UserRoleRepository,
    RO: RoleRepository,
    W: WebhookRepository,
    I: IdentityProviderRepository,
    CS: ClientScopeRepository,
    PM: ProtocolMapperRepository,
    CSM: ClientScopeMappingRepository,
    RU: RedirectUriRepository,
    CR: CredentialRepository,
    H: HasherRepository,
{
    pub(crate) async fn bootstrap_realm_impl(
        &self,
        identity: Identity,
        input: BootstrapRealmInput,
    ) -> Result<BootstrapRealmReport, CoreError> {
        let mut report = BootstrapRealmReport {
            realm_name: input.realm_name.clone(),
            ..Default::default()
        };

        // 1. Realm (idempotent). create_realm enforces the caller's policy and
        //    seeds the standard system clients + the `{realm}-realm` admin role.
        let realm = match self.realm_repository.get_by_name(&input.realm_name).await? {
            Some(r) => r,
            None => {
                let r = self
                    .create_realm(
                        identity.clone(),
                        CreateRealmInput {
                            realm_name: input.realm_name.clone(),
                        },
                    )
                    .await?;
                report.realm_created = true;
                r
            }
        };
        let realm_id = realm.id;

        // 2. Optional tenant-local auth-svc-m2m (confidential, service account).
        if input.include_tenant_m2m {
            let secret = generate_random_string();
            let (created, _) = self
                .ensure_tenant_m2m_client(realm_id, secret.clone())
                .await?;
            report.tenant_m2m_created = created;
            if created {
                report.tenant_m2m_secret = Some(secret);
            }
        }

        // 3. auth-svc-web confidential + callback redirect.
        let web_secret = generate_random_string();
        let (web_created, web_client) = self
            .ensure_client(CreateClientRequest {
                realm_id,
                name: "Auth Web Login (reserved)".to_string(),
                client_id: AUTH_SVC_WEB_CLIENT_ID.to_string(),
                secret: Some(web_secret.clone()),
                enabled: true,
                protocol: "openid-connect".to_string(),
                public_client: false,
                service_account_enabled: false,
                direct_access_grants_enabled: true,
                oauth_device_code_grant_enabled: false,
                client_type: ClientType::Confidential,
            })
            .await?;
        report.web_client_created = web_created;
        if web_created {
            report.web_client_secret = Some(web_secret);
        }
        if let Some(base) = input.auth_public_base_url.as_deref() {
            let callback = format!("{}/api/v1/auth/callback", base.trim_end_matches('/'));
            self.ensure_redirect_uri(web_client.id, &callback).await;
        }

        // 4. auth-svc-login-ui public + redirects.
        let (login_ui_created, login_ui_client) = self
            .ensure_client(CreateClientRequest {
                realm_id,
                name: "Login UI (reserved)".to_string(),
                client_id: AUTH_SVC_LOGIN_UI_CLIENT_ID.to_string(),
                secret: None,
                enabled: true,
                protocol: "openid-connect".to_string(),
                public_client: true,
                service_account_enabled: false,
                direct_access_grants_enabled: false,
                oauth_device_code_grant_enabled: false,
                client_type: ClientType::Public,
            })
            .await?;
        report.login_ui_client_created = login_ui_created;

        let mut login_ui_uris = vec![
            "http://127.0.0.1:3001/callback".to_string(),
            "http://localhost:3001/callback".to_string(),
        ];
        if let Some(origin) = input.portal_origin.as_deref() {
            login_ui_uris.push(format!("{}/callback", origin.trim_end_matches('/')));
        }
        for uri in &login_ui_uris {
            self.ensure_redirect_uri(login_ui_client.id, uri).await;
        }

        // 5. Product roles superadmin / admin / staff.
        for role_name in BOOTSTRAP_ROLES {
            let exists = self
                .role_repository
                .find_by_name(role_name.to_string(), realm_id.into())
                .await?
                .is_some();
            if !exists {
                self.role_repository
                    .create(CreateRoleRequest {
                        client_id: None,
                        description: None,
                        name: role_name.to_string(),
                        permissions: vec![],
                        realm_id,
                    })
                    .await?;
                report.roles_created.push(role_name.to_string());
            }
        }

        // 6. Bootstrap admin user with superadmin role + password.
        if let Some(admin) = input.bootstrap_admin {
            let user = match self
                .user_repository
                .get_by_username(admin.username.clone(), realm_id)
                .await
            {
                Ok(u) => u,
                Err(_) => {
                    let u = self
                        .user_repository
                        .create_user(CreateUserRequest {
                            realm_id,
                            client_id: None,
                            username: admin.username.clone(),
                            firstname: admin.firstname.clone(),
                            lastname: admin.lastname.clone(),
                            email: admin.email.clone(),
                            email_verified: true,
                            enabled: true,
                        })
                        .await?;
                    report.admin_user_created = true;
                    u
                }
            };

            let hash = self
                .hasher_repository
                .hash_password(&admin.password)
                .await
                .map_err(|e| CoreError::HashPasswordError(e.to_string()))?;
            // Ignore "already exists" — bootstrap is idempotent.
            let _ = self
                .credential_repository
                .create_credential(user.id, "password".to_string(), hash, "".into(), false)
                .await;

            if let Some(role) = self
                .role_repository
                .find_by_name("superadmin".to_string(), realm_id.into())
                .await?
            {
                // Ignore duplicate-assignment errors.
                let _ = self.user_role_repository.assign_role(user.id, role.id).await;
            }
        }

        // Backfill platform-operator cross-realm admin on pre-fork realms (idempotent).
        if let Some(role_id) = self.cross_realm_admin_role_for(&input.realm_name).await? {
            self.assign_platform_operator_realm_admin(&input.realm_name, role_id)
                .await?;
        }

        Ok(report)
    }

    /// Ensure tenant `auth-svc-m2m` exists with service account enabled (repairs legacy clients).
    async fn ensure_tenant_m2m_client(
        &self,
        realm_id: RealmId,
        create_secret: String,
    ) -> Result<(bool, Client), CoreError> {
        let client_id = super::platform::PLATFORM_M2M_CLIENT_ID.to_string();
        match self
            .client_repository
            .get_by_client_id(client_id.clone(), realm_id)
            .await
        {
            Ok(existing) => {
                let client = if !existing.service_account_enabled {
                    self.client_repository
                        .update_client(
                            existing.id,
                            UpdateClientRequest {
                                service_account_enabled: Some(true),
                                ..Default::default()
                            },
                        )
                        .await?
                } else {
                    existing
                };
                self.ensure_service_account_user(realm_id, &client).await?;
                Ok((false, client))
            }
            Err(_) => {
                let (created, client) = self
                    .ensure_client(CreateClientRequest {
                        realm_id,
                        name: "Auth Service M2M (reserved)".to_string(),
                        client_id,
                        secret: Some(create_secret),
                        enabled: true,
                        protocol: "openid-connect".to_string(),
                        public_client: false,
                        service_account_enabled: true,
                        direct_access_grants_enabled: false,
                        oauth_device_code_grant_enabled: false,
                        client_type: ClientType::Confidential,
                    })
                    .await?;
                if created {
                    self.ensure_service_account_user(realm_id, &client).await?;
                }
                Ok((created, client))
            }
        }
    }

    async fn ensure_service_account_user(
        &self,
        realm_id: RealmId,
        client: &Client,
    ) -> Result<(), CoreError> {
        if self.user_repository.get_by_client_id(client.id).await.is_ok() {
            return Ok(());
        }
        let service_account_username = format!("service-account-{}", client.client_id);
        let service_account_email = format!("{}@serviceaccount.local", client.client_id);
        let _ = self
            .user_repository
            .create_user(CreateUserRequest {
                realm_id,
                client_id: Some(client.id),
                username: service_account_username,
                firstname: Some("Service".to_string()),
                lastname: Some("Account".to_string()),
                email: Some(service_account_email),
                email_verified: true,
                enabled: true,
            })
            .await;
        Ok(())
    }

    /// Create the client if it does not already exist. Returns `(created, client)`.
    async fn ensure_client(
        &self,
        req: CreateClientRequest,
    ) -> Result<(bool, Client), CoreError> {
        match self
            .client_repository
            .get_by_client_id(req.client_id.clone(), req.realm_id)
            .await
        {
            Ok(existing) => Ok((false, existing)),
            Err(_) => {
                let created = self
                    .client_repository
                    .create_client(req)
                    .await
                    .map_err(|_| CoreError::CreateClientError)?;
                Ok((true, created))
            }
        }
    }

    /// Idempotently add a redirect URI to a client (best-effort).
    async fn ensure_redirect_uri(&self, client_id: Uuid, value: &str) {
        let existing = self
            .redirect_uri_repository
            .get_by_client_id(client_id)
            .await
            .unwrap_or_default();
        if existing.iter().any(|u| u.value == value) {
            return;
        }
        let _ = self
            .redirect_uri_repository
            .create_redirect_uri(client_id, value.to_string(), true)
            .await;
    }
}
