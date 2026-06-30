# Cross-realm administration

FerrisKey supports multi-tenant deployments where a **master** realm holds operators and automation identities that manage **tenant** realms. Cross-realm access is enforced through the standard role and permission model — there are no product-specific JWT claims or hardcoded automation client IDs.

## How it works

When a tenant realm is created via `POST /realms`, FerrisKey also creates in the **master** realm:

1. A system client named `{tenant}-realm` (client_id equals the role name).
2. A master-realm role bound to that client, initially granted `manage_realm`.
3. An assignment of that role to the user who created the tenant.

Master-realm identities (human admins or confidential clients with service accounts) can act on a tenant realm when they hold a master-realm role whose permissions apply to the tenant via the `{tenant}-realm` client binding.

Permission resolution uses `get_permission_for_target_realm` in the policy layer: for cross-realm calls, effective permissions come from the caller's roles that reference the target realm's `{tenant}-realm` client.

## Wiring automation (integrator responsibility)

FerrisKey does **not** auto-configure automation service accounts. External integrators (automation services, provisioning tools) must use standard admin APIs:

| Step | API |
|------|-----|
| Create tenant | `POST /realms` with `{ "name": "<tenant>" }` |
| Grant cross-realm permissions | `PATCH /realms/master/roles/{role_id}/permissions` on the auto-created `{tenant}-realm` role (add permissions such as `view_clients`, `manage_clients`, …) |
| Create automation client | `POST /realms/master/clients` — confidential client with `service_account_enabled: true` |
| Assign role to service account | `POST /realms/master/users/{sa_user_id}/roles/{role_id}` |
| Obtain token | `POST /realms/master/protocol/openid-connect/token` with `grant_type=client_credentials` |
| Call tenant APIs | e.g. `GET /realms/{tenant}/clients` with the bearer token |

Service account users are created automatically with username `service-account-{client_id}`. List users in the master realm to resolve the UUID for role assignment.

## Permission names

Permissions are string identifiers (see role permission catalog), for example:

- `manage_realm` — realm record management
- `view_clients` / `manage_clients` — read or manage clients in the target realm
- `view_users` / `manage_users` — user administration in the target realm

Grant only the permissions required for the automation task.

## Related endpoints

- Role permissions: `GET` and `PATCH /realms/{realm}/roles/{id}/permissions`
- Client secret (not exposed on list/get): `GET /realms/{realm}/clients/{uuid}/secret`
- User role assignment: `POST /realms/{realm}/users/{user_id}/roles/{role_id}`

See OpenAPI at `/swagger-ui` for the full admin surface.
