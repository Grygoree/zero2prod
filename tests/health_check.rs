#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn app.");
}

async fn spawn_app() -> Result<(), std::io::Error> {
    todo!()
}
