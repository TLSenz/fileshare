use fileshare::model::{LoginRequest, LoginResponse};

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