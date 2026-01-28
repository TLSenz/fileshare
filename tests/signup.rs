use axum::http::StatusCode;
use fileshare::configuration::get_config;
use fileshare::db::create_pool;
use fileshare::model::{SignupRequest};
use reqwest::header::CONTENT_TYPE;

#[tokio::test]
async fn test_health_check() {
    let request = reqwest::get("http://localhost:3000/")
        .await
        .expect("Could not Connect to API. Please ensure a instance is running");
    println!("{:?}", request);
    assert!(request.status().is_success());
}

#[tokio::test]
async fn test_sign_up() {
    let settings = get_config().expect("Could Not get Connection String for Connection String");
    let db_pool = create_pool(&settings.connection_string_database())
        .await
        .expect("Failed to get DB conn");
    let client = reqwest::Client::new();

    let test_user = SignupRequest::new(
        "Sven".to_string(),
        "2009Formel1".to_string(),
        "sven@zemp.email".to_string(),
    );

    let request = client
        .post("http://127.0.0.1:3000/api/signup")
        .header(CONTENT_TYPE, "application/json")
        .json(&test_user)
        .send()
        .await
        .expect("Could not Connect to Backend. PLease ensure a Instance is running");
    println!("{:?}", request);

    assert!(request.status().is_success());

    sqlx::query("Delete from users where name = 'Sven'")
        .execute(&db_pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn negative_test_sign_up() {
    let client = reqwest::Client::new();

    let mut signup_data: Vec<SignupRequest> = Vec::new();
    signup_data.push(SignupRequest::new(
        "".to_string(),
        "Sven".to_string(),
        "sven@test.email".to_string(),
    ));
    signup_data.push(SignupRequest::new(
        "Test".to_string(),
        "".to_string(),
        "sven@zemp.email".to_string(),
    ));
    signup_data.push(SignupRequest::new(
        "Test".to_string(),
        "Sven".to_string(),
        "svenzemp.email".to_string(),
    ));
    //signup_data.push(SignupRequest::new("Sven".to_string(), "Test".to_string(), "test@testemail".to_string()));

    for signup_request in signup_data {
        let request = client
            .post("http://127.0.0.1:3000/api/signup")
            .header(CONTENT_TYPE, "application/json")
            .json(&signup_request)
            .send()
            .await
            .expect("Could not Connect to Backend. PLease ensure a Instance is running");

        assert!(request.status().eq(&StatusCode::BAD_REQUEST));
    }
}
