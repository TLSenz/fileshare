extern crate core;

use reqwest::multipart::{Part, Form};
use std::fs;
use std::io::Read;
use axum::http::header::CONTENT_TYPE;
use fileshare::db::create_pool;
use fileshare::model::{LoginRequest, LoginResponse, SignupRequest};
use sqlx::*;
async fn login() -> String {
    let client = reqwest::Client::new();

    let credentials = LoginRequest::new("Test".to_string(), "Test".to_string(), "test@test.email".to_string());
    let response = client
        .post("http://127.0.0.1:3000/api/login")
        .header(CONTENT_TYPE,"application/json")
        .json(&credentials)
        .send()
        .await.expect("Could not Connect to Backend. PLease ensure a Instance is running");
    println!("{:?}",response);
    assert!(response.status().is_success());

    let response_json = response.json::<LoginResponse>().await.unwrap();
    response_json.token

}
#[tokio::test]
async fn test_upload() {
    let db_pool = create_pool("postgres://postgres:example@localhost/fileshare").await.expect("no connection to db");
    let token = login().await;
    let client = reqwest::Client::new();

    let file = fs::read("test_upload_files/hello.md").expect("Could not read file");
    let file_multipart = Part::bytes(file).file_name("hello.md").mime_str("text/markdown").expect("Could not create multipart");
    let multipart = Form::new().part("test_file12345",file_multipart);

    let response = client
        .post("http://127.0.0.1:3000/api/upload")
        .bearer_auth(token)
        .multipart(multipart)
        .send()
        .await
        .expect("Could not Connect to Backend. PLease ensure a Instance is running");

    println!("{:?}",response);

    assert!(response.status().is_success());

    let link = response.text().await.unwrap();

    println!("{}", link);

    let response = client
        .get(link)
        .send()
        .await
        .expect("Could not Connect to Backend. PLease ensure a Instance is running");

    assert!(response.status().is_success());
    fs::remove_file("content/test_file12345.markdown").expect("Could not delete file");
    
    sqlx::query("Delete from file where file_name = 'test_file12345'").fetch_all(&db_pool).await.unwrap();

}


