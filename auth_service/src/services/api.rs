use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::{Deserialize, Serialize};

pub enum AuthApiError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
    MissingToken,
    InvalidToken,
    IncorrectCredentials
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthApiError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthApiError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthApiError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthApiError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
            AuthApiError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthApiError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}