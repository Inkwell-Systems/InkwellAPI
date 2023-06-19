use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addr = spawn_app();
    let test_url = &format!("http://{}/health_check", &addr);

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

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Error binding to random port.");
    let port = listener.local_addr().unwrap().port();

    let server = inkwell_api::run(listener).expect("Failed to bind address.");

    // NOTES(calco): Non binding let, as we specifically DO NOT want to wait for completion.
    // (it never finishes)
    let sp = tokio::spawn(server);
    drop(sp);

    format!("127.0.0.1:{}", port)
}
