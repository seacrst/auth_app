use auth_service::App;

#[tokio::main]
async fn main() {
    let app = App::build("0.0.0.0:4000").await.expect("Failed to build app");
    app.run().await.expect("Failed to run app")
}
