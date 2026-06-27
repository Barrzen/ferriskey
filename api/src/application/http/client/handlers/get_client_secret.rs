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
use ferriskey_core::domain::client::ports::ClientService;
use ferriskey_core::domain::{
    authentication::value_objects::Identity, client::entities::GetClientInput,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GetClientSecretResponse {
    pub secret: Option<String>,
}

#[utoipa::path(
    get,
    path = "/{client_id}/secret",
    summary = "Get a client's secret",
    description = "Returns the confidential secret of a client. Requires view-clients permission.",
    params(
        ("realm_name" = String, Path, description = "Realm name"),
        ("client_id" = Uuid, Path, description = "Client ID"),
    ),
    tag = "client",
    responses(
        (status = 200, description = "Client secret retrieved successfully", body = GetClientSecretResponse),
        (status = 401, description = "Realm not found", body = ApiErrorResponse),
        (status = 403, description = "Insufficient permissions", body = ApiErrorResponse),
        (status = 404, description = "Client not found", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse),
    )
)]
pub async fn get_client_secret(
    Path((realm_name, client_id)): Path<(String, Uuid)>,
    State(state): State<AppState>,
    Extension(identity): Extension<Identity>,
) -> Result<Response<GetClientSecretResponse>, ApiError> {
    let secret = state
        .service
        .get_client_secret(
            identity,
            GetClientInput {
                client_id,
                realm_name,
            },
        )
        .await?;

    Ok(Response::OK(GetClientSecretResponse { secret }))
}
