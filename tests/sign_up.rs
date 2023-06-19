use std::collections::HashMap;

#[tokio::test]
async fn sign_up_valid_json() {
    // Arrange
    let addr = inkwell_api::spawn_app();
    let test_url = &format!("http://{}/sign_up", &addr);

    let mut user_data = HashMap::new();
    user_data.insert("display_name", "Calcopod");
    user_data.insert("email", "calcopoddev@gmail.com");

    // Act
    let client = reqwest::Client::new();
    let response = client
        .post(test_url)
        .json(&user_data)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn sign_up_invalid_json() {
    // Arrange
    let addr = inkwell_api::spawn_app();
    let test_url = &format!("http://{}/sign_up", &addr);

    let test_cases = vec![
        ("", "", "Missing both fields."),
        ("email", "calcopoddev@gmail.com", "Missing display name."),
        ("display_name", "Calcopod", "Missing email."),
    ];

    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    for (key, value, error_message) in test_cases {
        map.clear();
        map.insert(key, value);

        // Act
        let response = client
            .post(test_url)
            .json(&map)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail when the sign_up payload was: {}",
            error_message
        );
    }
}
