use std::{borrow::{Borrow, BorrowMut}, ops::Deref};

use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{app_state::AppState, domain::User, services::UserStore};

#[derive(Deserialize, Validate, Debug)]
pub struct SignupRequest {
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

pub async fn signup(State(
    state): State<AppState>, 
    Json(SignupRequest {email, password, ..}
    ): Json<SignupRequest>) -> impl IntoResponse {
    let user = User::new("", &password, &email);

    let mut user_store = state.user_store.write().await;
    user_store.borrow_mut().add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}


#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
