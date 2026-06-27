use crate::application::http::realm::validators::ImportRealmValidator;
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
use ferriskey_core::domain::realm::ports::{BootstrapRealmReport, RealmService};
use ferriskey_core::domain::realm::templates::resolve_template;

#[utoipa::path(
    post,
    path = "/{realm_name}/import",
    tag = "realm",
    summary = "Bootstrap a tenant realm from a named template",
    description = "Resolves a provisioning template (default `tenant-default`) into a bootstrap input and applies it idempotently.",
    request_body = ImportRealmValidator,
    params(("realm_name" = String, Path, description = "Realm name")),
    responses(
        (status = 200, description = "Realm imported successfully", body = BootstrapRealmReport),
        (status = 400, description = "Invalid request data or unknown template", body = ApiErrorResponse),
        (status = 403, description = "Insufficient permissions", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    ),
)]
pub async fn import_realm(
    Path(realm_name): Path<String>,
    State(state): State<AppState>,
    Extension(identity): Extension<Identity>,
    ValidateJson(payload): ValidateJson<ImportRealmValidator>,
) -> Result<Response<BootstrapRealmReport>, ApiError> {
    let mut input = resolve_template(&payload.template, realm_name).ok_or_else(|| {
        ApiError::BadRequest(format!("unknown realm template: {}", payload.template).into())
    })?;

    // Caller overrides on top of the template skeleton.
    input.bootstrap_admin = payload.bootstrap_admin.map(Into::into);
    if payload.auth_public_base_url.is_some() {
        input.auth_public_base_url = payload.auth_public_base_url;
    }
    if payload.portal_origin.is_some() {
        input.portal_origin = payload.portal_origin;
    }

    let report = state
        .service
        .bootstrap_realm(identity, input)
        .await?;

    Ok(Response::OK(report))
}
