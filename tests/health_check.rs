#[tokio::test]
#[allow(unused_must_use)]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[allow(clippy::let_underscore_future)]
async fn spawn_app() {
    let server = zero2prod::run().await.expect("Failed to bind address.");

    let _ = tokio::spawn(server);
}
