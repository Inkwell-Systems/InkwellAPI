use inkwell_api::configuration::get_config;
use sqlx::{Connection, PgConnection};
use std::collections::HashMap;

#[tokio::test]
async fn sign_up_valid_json() {
    // Arrange
    let addr = inkwell_api::spawn_app().await;
    let test_url = &format!("http://{}/sign_up", &addr);

    let mut user_data = HashMap::new();
    user_data.insert("display_name", "Calcopod");
    user_data.insert("email", "calcopoddev@gmail.com");

    let config = get_config().expect("Failed to load config: ");
    let connect_str = config.db_settings.get_connection_string();

    let mut connection = PgConnection::connect(&connect_str)
        .await
        .expect("Connection to Postgres DB failed: ");

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

    let saved =
        sqlx::query!("SELECT display_name, email, created_at FROM users")
            .fetch_one(&mut connection)
            .await
            .expect("Failed to fetch saved user.");

    assert_eq!(saved.display_name, "Calcopod");
    assert_eq!(saved.email, "calcopoddev@gmail.com");
}

#[tokio::test]
async fn sign_up_invalid_json() {
    // Arrange
    let addr = inkwell_api::spawn_app().await;
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
