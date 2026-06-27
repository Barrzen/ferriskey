use crate::application::http::server::{
    api_entities::{
        api_error::{ApiError, ApiErrorResponse},
        response::Response,
    },
    app_state::AppState,
};
use axum::{
    Extension,
    extract::{Path, State},
};
use ferriskey_core::domain::authentication::value_objects::Identity;
use ferriskey_core::domain::role::ports::RoleService;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GetRolePermissionsResponse {
    pub data: Vec<String>,
}

#[utoipa::path(
    get,
    summary = "Get the permissions of a role in a realm",
    path = "/{role_id}/permissions",
    tag = "role",
    params(
        ("realm_name" = String, Path, description = "Realm name"),
        ("role_id" = Uuid, Path, description = "Role ID"),
    ),
    responses(
        (status = 200, description = "Role permissions retrieved successfully", body = GetRolePermissionsResponse),
        (status = 403, description = "Insufficient permissions", body = ApiErrorResponse),
        (status = 404, description = "Role not found", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
pub async fn get_role_permissions(
    Path((realm_name, role_id)): Path<(String, Uuid)>,
    State(state): State<AppState>,
    Extension(identity): Extension<Identity>,
) -> Result<Response<GetRolePermissionsResponse>, ApiError> {
    let permissions = state
        .service
        .get_role_permissions(identity, realm_name, role_id)
        .await
        .map_err(ApiError::from)?;

    Ok(Response::OK(GetRolePermissionsResponse { data: permissions }))
}
