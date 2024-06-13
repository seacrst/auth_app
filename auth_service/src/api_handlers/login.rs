use axum::{
    extract::State, 
    http::StatusCode, 
    response::IntoResponse, 
    Json
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app::state::AppState, 
    services::{api::AuthApiError, auth::generate_auth_cookie}, 
    user::{Email, Password}
};

#[derive(Deserialize)]
pub struct LoginReq {
    email: String,
    password: String,
}

pub async fn login(
    State(state): State<AppState>, 
    jar: CookieJar, 
    Json(req): Json<LoginReq>
) -> (CookieJar, Result<impl IntoResponse, AuthApiError>) {
    let user_email = match Email::parse(req.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthApiError::InvalidCredentials))
    };
    let user_password = match Password::parse(req.password) {
        Ok(pw) => pw,
        Err(_) => return (jar, Err(AuthApiError::InvalidCredentials))
    };

    let store = &state.user_store.read().await;
    
    if store.validate_user(&user_email, &user_password).await.is_err() {
        return (jar, Err(AuthApiError::IncorrectCredentials));
    }
    
    let user = match store.get_user(&user_email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthApiError::IncorrectCredentials)),
    };

    let auth_cookie = match generate_auth_cookie(&user.email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthApiError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}