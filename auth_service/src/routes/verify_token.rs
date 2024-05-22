use axum::response::IntoResponse;
use reqwest::StatusCode;

pub async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
