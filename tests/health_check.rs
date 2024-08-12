use newsletter::startup;

#[tokio::test]
async fn health_check_works() {
    // No .await, no .expect
    let test_app = startup::spawn_app().await;
    let client = reqwest::Client::new();

    // Send request
    let response = client
        .get(format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
