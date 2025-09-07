use reqwest::header::CONTENT_TYPE;
use fileshare::model::SignupRequest;

#[tokio::test]
async  fn test_health_check() {
    let request = reqwest::get("http://localhost:3000/")
        .await
        .expect("Could not Connect to API. Please ensure a instance is running");
    println!("{:?}", request);
    assert!(request.status().is_success());
}

#[tokio::test]
async fn test_sign_up() {
    let client = reqwest::Client::new();
    
    let test_user = SignupRequest::new("Sven".to_string(), "2009Formel1".to_string(), "sven@zemp.email".to_string());
    
    let request = client.post("http://127.0.0.1:3000/api/signup")
        .header(CONTENT_TYPE, "application/json")
        .json(&test_user)
        .send()
        .await
        .expect("Could not Connect to Backend. PLease ensure a Instance is running");
    println!("{:?}", request);
    assert!(request.status().is_success())
}