use crate::utils::TestApp;

#[tokio::test]
async fn login_returns_status_code() {
    let app = TestApp::new().await;

    let response = app.get_login().await;

    assert_eq!(response.status().as_u16(), 200);
}
