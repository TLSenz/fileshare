use axum::http::header::CONTENT_TYPE;
use reqwest::StatusCode;
use fileshare::model::{LoginRequest, LoginResponse, SignupRequest};

#[tokio::test]
async fn test_login() {

    let client = reqwest::Client::new();

    let login_data = LoginRequest::new("Test".to_string(), "Test".to_string(), "test@test.email".to_string());


    let responnse = client
        .post("http://127.0.0.1:3000/api/login")
        .header("Content-Type","application/json")
        .json(&login_data)
        .send()
        .await.expect("Could not Connect to Backend. PLease ensure a Instance is running");


    let response_json = responnse.json::<LoginResponse>().await.unwrap();

    assert!(response_json.token.len() > 0);
}

#[tokio::test]
async fn negative_test_login() {
    let client = reqwest::Client::new();

    let mut login_data:Vec<LoginRequest> = Vec::new();
    login_data.push(LoginRequest::new("".to_string(), "Sven".to_string(), "test@test.email".to_string()));
    login_data.push(LoginRequest::new("Test".to_string(), "".to_string(), "sven@zemp.email".to_string()));
    login_data.push(LoginRequest::new("Sven".to_string(), "Test".to_string(), "test@test.email".to_string()));

    for loginRequest in login_data.iter() {
        let response = client.post("http://127.0.0.1:3000/api/login")
            .header("Content-Type", "application/json")
            .json(loginRequest)
            .send()
            .await
            .expect("Could not connect to Backend");

        assert!(response.status().eq(&StatusCode::UNAUTHORIZED))
    }
}
