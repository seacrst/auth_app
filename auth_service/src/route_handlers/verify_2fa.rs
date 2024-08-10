use axum::{
    extract::State, 
    response::IntoResponse, 
    Json
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app::state::AppState, 
    services::{
        api::AuthApiError, 
            auth::generate_auth_cookie, 
            two_fa::{LoginId, TwoFaCode}
        }, 
    user::Email
};

#[derive(Deserialize, Debug)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "lodinId")]
    pub login_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String 
}

pub async fn verify_2fa(
    jar: CookieJar,
    State(state): State<AppState>,
    Json(request): Json<Verify2FARequest>
) -> (CookieJar, Result<impl IntoResponse, AuthApiError>) {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthApiError::InvalidCredentials))
    };

    let login_id_req = match LoginId::parse(request.login_id) {
        Ok(id) => id,
        Err(_) => return (jar, Err(AuthApiError::InvalidCredentials))
    };

    let two_fa_code_req = match TwoFaCode::parse(request.two_fa_code) {
        Ok(code) => code,
        Err(_) => return (jar, Err(AuthApiError::InvalidToken))
    };

    let mut two_fa_code_store = state.two_fa_code.write().await;

    let two_fa_tup = match two_fa_code_store.get_code(&email).await {
        Ok(tup) => tup,
        Err(_) => return (jar, Err(AuthApiError::IncorrectCredentials))
    };

    if two_fa_code_store.remove_code(&email).await.is_err() {
        return (jar, Err(AuthApiError::UnexpectedError));
    }

    let cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthApiError::UnexpectedError)),
    };

    match two_fa_tup {
        (id, code) if !login_id_req.eq(&id) || !two_fa_code_req.eq(&code) => (
            jar, 
            Err(AuthApiError::IncorrectCredentials)
        ),
        _ =>  (jar.add(cookie), Ok(()))
    }
}
