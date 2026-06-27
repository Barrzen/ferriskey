# Platform integration

This fork adds a small set of endpoints and behaviours so an external auth-service
(the multi-tenant facade in `barrzen-auth-service`) can drive FerrisKey as a
**platform operator**: one reserved M2M client on the `master` realm that
provisions and administers every tenant realm.

These additions are **backward compatible** — existing realm, client, role, and
token behaviour is unchanged unless you opt into the new routes.

## Platform operator

The reserved client id is `auth-svc-m2m` on the `master` realm. When this client
has a service account enabled, it is treated as the platform operator.

- On **realm creation** (`POST /realms`), the per-tenant `{realm}-realm` client
  role is granted the full cross-realm admin permission set
  (`manage_realm`, `manage_clients`, `manage_roles`, `manage_users`,
  `view_clients`, `query_clients`) instead of only `manage_realm`. If the master
  `auth-svc-m2m` service account exists, the same role is assigned to it
  (best-effort, idempotent) so it can administer the new tenant realm without a
  tenant-scoped token.
- Access tokens issued to `auth-svc-m2m` on `master` (via
  `grant_type=client_credentials`) carry two extra claims:

  ```json
  { "platform_operator": true, "managed_realms": ["*"] }
  ```

  The claim is scoped to `master` so a tenant-local client that happens to share
  the `auth-svc-m2m` id never receives it.

Source: `core/src/domain/realm/platform.rs`, `core/src/domain/realm/services.rs`,
`core/src/domain/authentication/services.rs`.

## Bootstrap API

Idempotent tenant provisioning that mirrors the auth-api `create-realm.sh`
blueprint (reserved clients + product roles + optional bootstrap admin).

```
POST {root}/realms/{realm_name}/bootstrap
Authorization: Bearer <admin or platform-operator token>
Content-Type: application/json

{
  "include_tenant_m2m": true,
  "auth_public_base_url": "https://auth.example.com",
  "portal_origin": "https://portal.example.com",
  "bootstrap_admin": {
    "username": "superadmin",
    "password": "superadmin",
    "email": "admin@example.com",
    "firstname": "Super",
    "lastname": "Admin"
  }
}
```

What it provisions (each step is idempotent — safe to re-run):

| Step | Detail |
|------|--------|
| Realm | created via `create_realm` if missing |
| `auth-svc-m2m` (tenant) | optional, when `include_tenant_m2m` is true — confidential + service account |
| `auth-svc-web` | confidential; callback redirect seeded from `auth_public_base_url` |
| `auth-svc-login-ui` | public; redirects seeded from `portal_origin` |
| Roles | `superadmin`, `admin`, `staff` |
| Bootstrap admin | created with the `superadmin` role when `bootstrap_admin` is present |

Response is a `BootstrapRealmReport`:

```json
{
  "realm_name": "acme",
  "realm_created": true,
  "web_client_created": true,
  "login_ui_client_created": true,
  "tenant_m2m_created": true,
  "roles_created": ["superadmin", "admin", "staff"],
  "admin_user_created": true,
  "web_client_secret": "…",
  "tenant_m2m_secret": "…"
}
```

Generated client secrets are only populated on **first** creation of the
respective client; re-runs return `null` for already-existing clients.

Source: `core/src/domain/realm/bootstrap.rs`,
`api/src/application/http/realm/handlers/bootstrap_realm.rs`.

## Template import

`import` resolves a named template into a bootstrap input and applies it. The
only template today is `tenant-default`.

```
POST {root}/realms/{realm_name}/import
Authorization: Bearer <token>
Content-Type: application/json

{
  "template": "tenant-default",
  "bootstrap_admin": { "username": "superadmin", "password": "superadmin" },
  "auth_public_base_url": "https://auth.example.com",
  "portal_origin": "https://portal.example.com"
}
```

`template` defaults to `tenant-default`. Unknown template names return `400`.
The response is the same `BootstrapRealmReport` as `/bootstrap`.

Source: `core/src/domain/realm/templates.rs`,
`api/src/application/http/realm/handlers/import_realm.rs`.

## Permissions GET symmetry

- `GET {root}/realms/{realm_name}/roles/{role_id}/permissions` returns a role's
  permissions (alongside the existing `PATCH`). Behind the `view-role` policy.
- `get_user_permissions` no longer rejects a caller whose home realm differs from
  the requested realm; cross-realm resolution is delegated to the policy
  (`get_permission_for_target_realm`). This lets a platform operator read tenant
  permissions.

Source: `core/src/domain/role/services.rs`,
`api/src/application/http/role/handlers/get_role_permissions.rs`,
`core/src/domain/user/services.rs`.

## Client management

- `GET {root}/realms/{realm_name}/clients/{client_id}/secret` returns a client's
  confidential secret behind the `view-clients` policy and records a
  `client_secret_viewed` security event.
- `GET /clients` and `GET /clients/{id}` now **redact** the `secret` field — use
  the dedicated secret endpoint above.
- `POST /clients/{id}/redirects` is idempotent: posting a redirect URI value that
  already exists for the client returns the existing entry instead of erroring.

Source: `core/src/domain/client/services.rs`,
`api/src/application/http/client/handlers/get_client_secret.rs`.

## Tests

Both suites are `#[ignore]` and require a Postgres reachable via the
`DATABASE_*` env vars (same convention as `token_basic_auth_test.rs`):

```
cargo test -p ferriskey-api -- --ignored
```

- `api/tests/platform_operator_test.rs` — platform operator claims + cross-realm
  client listing.
- `api/tests/bootstrap_realm_test.rs` — idempotent bootstrap, role seeding, and
  template import.
