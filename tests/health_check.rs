fn spawn_app() {
    let server = news_letter::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
