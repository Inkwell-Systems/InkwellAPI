﻿// TODO(calco): This is highly questionable. Should be replaced
// with a separate cargo library called utils or something.
#[path = "./g.rs"]
mod g;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = g::spawn_app().await;
    let test_url = &format!("http://{}/health_check", &app.address);

    // Act
    let client = reqwest::Client::new();
    let response = client
        .get(test_url)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
