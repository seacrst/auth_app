
use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{app::state::AppState, services::api::AuthApiError, user::{Email, Password, User}};

#[derive(Deserialize, Validate, Debug)]
pub struct SignupRequest {
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

pub async fn signup(State(state): State<AppState>, Json(SignupRequest {email, password, requires_2fa}): Json<SignupRequest>) -> Result<impl IntoResponse, AuthApiError> {
    let user_email = Email::parse(email.clone())
        .map_err(|_| AuthApiError::InvalidCredentials)?;
    let user_password = Password::parse(password.clone())
        .map_err(|_| AuthApiError::InvalidCredentials)?;

    let user = User { 
        email: user_email, 
        password: user_password, 
        requires_2fa
    };

    let mut user_store = state.user_store.write().await;

    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthApiError::UserAlreadyExists);
    }

    if user_store.add_user(user).await.is_err() {
        return Err(AuthApiError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}


#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
