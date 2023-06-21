// TODO(calco): This is highly questionable. Should be replaced
// with a separate cargo library called utils or something.
#[path = "./g.rs"]
mod g;

use std::collections::HashMap;

#[tokio::test]
async fn sign_up_valid_json() {
    // Arrange
    let app = g::spawn_app().await;
    let test_url = &format!("http://{}/sign_up", &app.address);

    let mut user_data = HashMap::new();
    user_data.insert("display_name", "user_insert_test");
    user_data.insert("email", "user_insert_test@gmail.com");
    user_data.insert("profile_url", "user_insert_test/profile");

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
            .fetch_one(&app.db_pool)
            .await
            .expect("Failed to fetch saved user.");

    assert_eq!(saved.display_name, user_data["display_name"]);
    assert_eq!(saved.email, user_data["email"]);

    // !REVERT
}

#[tokio::test]
async fn sign_up_invalid_json() {
    // Arrange
    let app = g::spawn_app().await;
    let test_url = &format!("http://{}/sign_up", &app.address);

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
