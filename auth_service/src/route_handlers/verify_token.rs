use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{app::state::AppState, services::{api::AuthApiError, auth::validate_token}};

pub async fn verify_token(
    State(state): State<AppState>,
    Json(VerifyTokenReq {token}): Json<VerifyTokenReq>
) -> Result<StatusCode, AuthApiError> {
    match validate_token(&token, state.banned_token_store.clone()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthApiError::InvalidToken)
    }
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenReq {
    token: String
}
