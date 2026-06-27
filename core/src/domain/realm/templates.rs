//! Realm provisioning templates.
//!
//! Templates name a reusable tenant layout (reserved clients + roles + default
//! redirect patterns) that `import_realm` resolves into a [`BootstrapRealmInput`]
//! before delegating to `bootstrap_realm`.

use crate::domain::realm::ports::BootstrapRealmInput;

/// Canonical tenant template name.
pub const TENANT_DEFAULT_TEMPLATE: &str = "tenant-default";

/// Reserved OAuth client ids provisioned by the `tenant-default` template.
pub const TENANT_DEFAULT_CLIENT_IDS: [&str; 3] = [
    super::platform::PLATFORM_M2M_CLIENT_ID,
    super::bootstrap::AUTH_SVC_WEB_CLIENT_ID,
    super::bootstrap::AUTH_SVC_LOGIN_UI_CLIENT_ID,
];

/// Product roles seeded by the `tenant-default` template.
pub const TENANT_DEFAULT_ROLES: [&str; 3] = super::bootstrap::BOOTSTRAP_ROLES;

/// Default login-ui redirect patterns seeded by the `tenant-default` template.
pub const TENANT_DEFAULT_REDIRECT_PATTERNS: [&str; 2] = [
    "http://127.0.0.1:3001/callback",
    "http://localhost:3001/callback",
];

/// Resolve a named template into a [`BootstrapRealmInput`] skeleton. The caller
/// can override the admin / URL fields before invoking `bootstrap_realm`.
/// Returns `None` for unknown template names.
pub fn resolve_template(name: &str, realm_name: String) -> Option<BootstrapRealmInput> {
    match name {
        TENANT_DEFAULT_TEMPLATE => Some(BootstrapRealmInput {
            realm_name,
            bootstrap_admin: None,
            include_tenant_m2m: false,
            auth_public_base_url: None,
            portal_origin: None,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_known_template() {
        let input = resolve_template(TENANT_DEFAULT_TEMPLATE, "acme".to_string())
            .expect("tenant-default should resolve");
        assert_eq!(input.realm_name, "acme");
        assert!(!input.include_tenant_m2m);
    }

    #[test]
    fn unknown_template_is_none() {
        assert!(resolve_template("nope", "acme".to_string()).is_none());
    }
}
