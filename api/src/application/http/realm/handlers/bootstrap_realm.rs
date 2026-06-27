use crate::application::http::realm::validators::{BootstrapAdminValidator, BootstrapRealmValidator};
use crate::application::http::server::api_entities::api_error::{
    ApiError, ApiErrorResponse, ValidateJson,
};
use crate::application::http::server::api_entities::response::Response;
use crate::application::http::server::app_state::AppState;
use axum::{
    Extension,
    extract::{Path, State},
};
use ferriskey_core::domain::authentication::value_objects::Identity;
use ferriskey_core::domain::realm::ports::{
    BootstrapAdminInput, BootstrapRealmInput, BootstrapRealmReport, RealmService,
};

impl From<BootstrapAdminValidator> for BootstrapAdminInput {
    fn from(v: BootstrapAdminValidator) -> Self {
        Self {
            username: v.username,
            password: v.password,
            email: v.email,
            firstname: v.firstname,
            lastname: v.lastname,
        }
    }
}

#[utoipa::path(
    post,
    path = "/{realm_name}/bootstrap",
    tag = "realm",
    summary = "Idempotently bootstrap a tenant realm",
    description = "Provisions reserved clients, product roles, and an optional bootstrap admin for a tenant realm. Safe to re-run.",
    request_body = BootstrapRealmValidator,
    params(("realm_name" = String, Path, description = "Realm name")),
    responses(
        (status = 200, description = "Realm bootstrapped successfully", body = BootstrapRealmReport),
        (status = 400, description = "Invalid request data", body = ApiErrorResponse),
        (status = 403, description = "Insufficient permissions", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    ),
)]
pub async fn bootstrap_realm(
    Path(realm_name): Path<String>,
    State(state): State<AppState>,
    Extension(identity): Extension<Identity>,
    ValidateJson(payload): ValidateJson<BootstrapRealmValidator>,
) -> Result<Response<BootstrapRealmReport>, ApiError> {
    let report = state
        .service
        .bootstrap_realm(
            identity,
            BootstrapRealmInput {
                realm_name,
                bootstrap_admin: payload.bootstrap_admin.map(Into::into),
                include_tenant_m2m: payload.include_tenant_m2m,
                auth_public_base_url: payload.auth_public_base_url,
                portal_origin: payload.portal_origin,
            },
        )
        .await?;

    Ok(Response::OK(report))
}
