//! Platform operator helpers.
//!
//! The auth-service fronting FerrisKey runs a single reserved M2M client on the
//! `master` realm (`auth-svc-m2m`) that needs to manage every tenant realm. This
//! module centralises the identifiers and the cross-realm permission set that the
//! platform operator is granted so that realm provisioning stays in sync.

use crate::domain::client::entities::Client;
use crate::domain::realm::entities::RealmId;
use crate::domain::role::entities::permission::Permissions;

/// Reserved client id of the master-realm M2M service account that auth-api uses
/// to manage tenant realms. Must match `auth-config`'s `reserved.rs`.
pub const PLATFORM_M2M_CLIENT_ID: &str = "auth-svc-m2m";

/// Permissions assigned to the per-tenant `{realm}-realm` client role so the
/// platform operator can fully administer the tenant realm (clients, roles,
/// users) rather than only `manage_realm`.
pub fn cross_realm_admin_permissions() -> Vec<String> {
    Permissions::to_names(&[
        Permissions::ManageRealm,
        Permissions::ManageClients,
        Permissions::ManageRoles,
        Permissions::ManageUsers,
        Permissions::ViewClients,
        Permissions::QueryClients,
    ])
}

/// Returns `true` when the given client is the reserved platform operator M2M
/// service account on the master realm.
pub fn is_platform_operator_client(client: &Client, master_realm_id: RealmId) -> bool {
    client.client_id == PLATFORM_M2M_CLIENT_ID
        && client.service_account_enabled
        && client.realm_id == master_realm_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_realm_admin_permissions_include_management_set() {
        let perms = cross_realm_admin_permissions();
        for expected in [
            "manage_realm",
            "manage_clients",
            "manage_roles",
            "manage_users",
            "view_clients",
            "query_clients",
        ] {
            assert!(
                perms.iter().any(|p| p == expected),
                "missing permission {expected}"
            );
        }
    }

    #[test]
    fn detects_platform_operator_only_on_master() {
        let master = RealmId::new(uuid::Uuid::new_v4());
        let other = RealmId::new(uuid::Uuid::new_v4());

        let mut client = Client::from_realm_and_client_id(master, PLATFORM_M2M_CLIENT_ID.to_string());
        client.service_account_enabled = true;
        assert!(is_platform_operator_client(&client, master));

        // Wrong realm.
        assert!(!is_platform_operator_client(&client, other));

        // Service account disabled.
        client.service_account_enabled = false;
        assert!(!is_platform_operator_client(&client, master));

        // Wrong client id.
        let mut other_client =
            Client::from_realm_and_client_id(master, "some-other-client".to_string());
        other_client.service_account_enabled = true;
        assert!(!is_platform_operator_client(&other_client, master));
    }
}
