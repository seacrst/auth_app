use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

pub async fn signup(Json(_request): Json<SignupRequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}


#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
