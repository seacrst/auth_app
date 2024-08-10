use axum::{
    extract::State, 
    http::StatusCode, 
    response::IntoResponse, 
    Json
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app::{email_client::SendEmail, state::AppState}, services::{api::AuthApiError, auth::generate_auth_cookie, two_fa::{LoginId, TwoFaCode}}, user::{Email, Password}
};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    SingleFactorAuth,
    TwoFactorAuth(TwoFactorAuthResponse)
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginId")]
    pub login_id: String,
}

pub async fn login(
    State(state): State<AppState>, 
    jar: CookieJar, 
    Json(req): Json<LoginRequest>
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

    match user.requires_2fa {
        true => handle_2fa(jar, &user.email, &state).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}


async fn handle_2fa(jar: CookieJar, email: &Email, state: &AppState) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthApiError>,
) {
    let login_id = LoginId::default();
    let two_fa_code = TwoFaCode::default();

    let added_code = state.two_fa_code
        .write()
        .await
        .add_code(email.clone(), login_id.clone(), two_fa_code.clone())
        .await
        .is_ok();

    if !added_code {
        return (jar, Err(AuthApiError::UnexpectedError));
    }

    let email_detes = SendEmail {
        recipient: email,
        subject: "2FA Code",
        content: two_fa_code.as_ref()
    };

    let sent = state.email_client
        .send_email(email_detes.recipient, email_detes.subject, email_detes.content)
        .await
        .is_ok();

    if !sent {
        return (jar, Err(AuthApiError::UnexpectedError));
    }

    let res = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: String::from("2FA required"),
        login_id: login_id.as_ref().to_string()
    }));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, res)))
}

async fn handle_no_2fa(email: &Email, jar: CookieJar) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthApiError>,
) {
    let auth_cookie = match generate_auth_cookie(email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthApiError::UnexpectedError)),
    };

    let jar = jar.add(auth_cookie);
    (jar, Ok((StatusCode::OK, Json(LoginResponse::SingleFactorAuth))))
}