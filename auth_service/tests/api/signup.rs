use crate::utils::{get_random_email, TestApp};
use auth_service::{
    api_handlers::SignupResponse,
    services::api::ErrorResponse
};
use serde_json::json;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true,
            "requires2FA": "false"
        }),
        serde_json::json!(
            {
                "email": random_email,
                "password": "pwd",
                "requires2FA": "true"
            }
        ),
        serde_json::json!(
            {
                "email": random_email,
                "password": "pwd",
                "requires2FA": "true"
            }
        ),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let test_app = TestApp::new().await;
    let body = serde_json::json!(
        {
            "email": "test_user@email.com",
            "password": "12341234",
            "requires2FA": true
        }
    );
    let response = test_app.post_signup(&body).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let test_app = TestApp::new().await;

    let json_invalid_email_empty = json!({
            "email": "",
            "password": "1234abcd",
            "requires2FA": false,
        });

    let json_invalid_email_no_at = json!({
            "email": "invalidemail",
            "password": "1234abcd",
            "requires2FA": false,
        });

    let json_invalid_pw_empty = json!({
        "email": "valid@email.com",
        "password": "",
        "requires2FA": false,
    });
    
    let json_invalid_pw_less_8 = json!({
            "email": "valid@email.com",
            "password": "1234",
            "requires2FA": false,
        });


    let req_tups = [
        (json_invalid_email_empty, "Email is empty"), 
        (json_invalid_email_no_at, "Email have to include, '@' symbol"), 
        (json_invalid_pw_empty, "password is empty"), 
        (json_invalid_pw_less_8, "password is short")
    ];

    for (req, reason) in req_tups.iter() {
        let res = test_app.post_signup(req).await;
        assert_eq!(res.status().as_u16(), 400, "Failed to resolve credentials: {:?}", reason);

        assert_eq!(res.json::<ErrorResponse>()
        .await
        .expect("Could not deserialize response body to ErrorResponse")
        .error,
    "Invalid credentials".to_owned())
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "abcd1234",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}