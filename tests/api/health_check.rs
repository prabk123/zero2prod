use crate::helpers::spawn_app;

#[actix_rt::test]
async fn health_check_works() {
    /*
    - checks if endpoint is exposed at /health_check
    - checks if endpoint is behind a GET request
    - checks if endpoint returns a 200 status code
    - checks if the response has no body
    */
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
