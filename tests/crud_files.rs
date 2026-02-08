use std::fs;
use axum::http::header::CONTENT_TYPE;
use axum::routing::delete;
use reqwest::multipart::{Form, Part};
use fileshare::configuration::get_config;
use fileshare::db::create_pool;
use fileshare::model::{LoginRequest, LoginResponse, SignupRequest, UploadResponse};

async fn login() -> String {
    let client = reqwest::Client::new();
    let signup_data = SignupRequest::new(
        "Test".to_string(),
        "Test".to_string(),
        "test@test.email".to_string(),
    );
    client
        .post("http://127.0.0.1:3000/api/signup")
        .header("Content-Type", "application/json")
        .json(&signup_data)
        .send()
        .await
        .expect("Could not Connect");

    let credentials = LoginRequest::new(
        "Test".to_string(),
        "Test".to_string(),
        "test@test.email".to_string(),
    );
    let response = client
        .post("http://127.0.0.1:3000/api/login")
        .header(CONTENT_TYPE, "application/json")
        .json(&credentials)
        .send()
        .await
        .expect("Could not Connect to Backend. PLease ensure a Instance is running");
    println!("{:?}", response);
    assert!(response.status().is_success());

    let response_json = response.json::<LoginResponse>().await.unwrap();
    response_json.token
}


async fn upload_file_delete_test() -> String {
    let settings = get_config().expect("could Not get Config");
    let db_pool = create_pool(&settings.connection_string_database())
        .await
        .expect("no connection to db");
    let token = login().await;
    let client = reqwest::Client::new();

    let file = fs::read("test_upload_files/hello.md").expect("Could not read file");
    let file_multipart = Part::bytes(file)
        .file_name("test_delete.md")
        .mime_str("text/markdown")
        .expect("Could not create multipart");
    let multipart = Form::new().part("test_file12345", file_multipart);

    let response = client
        .post("http://127.0.0.1:3000/api/upload")
        .bearer_auth(token)
        .multipart(multipart)
        .send()
        .await
        .expect("Could not Connect to Backend. PLease ensure a Instance is running");

    println!("{:?}", response);

    assert!(response.status().is_success());

    let delete_token = response.json::<UploadResponse>().await.unwrap().delete_token;

    delete_token
}
#[tokio::test]
async fn test_delete_file() {

    let jwt_token = login().await;
    let client = reqwest::Client::new();
    let delete_token = upload_file_delete_test().await;
    let response = client
        .delete(format!("http://127.0.0.1:3000/api/delete/{}", delete_token))
        .bearer_auth(jwt_token)
        .send()
        .await
        .expect("Could not Connect to Backend. PLease ensure a Instance is running");
    assert!(response.status().is_success());
}