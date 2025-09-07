use crate::fil


#[tokio::test]
async  fn test_health_check() {
    let request = reqwest::get("http://localhost:3000/")
        .await
        .expect("Could not Connect to API. Please ensure a instance is running");
    
    assert!(request.status().is_success());
}

#[tokio::test]
async fn test_sign_up() {
    
    let test_user = SignupRequest::new()
    
    let request = reqwest::get("localhost:3000/api/signup")
        .await
        .expect("Could not Connect to Backend. PLease ensure a Instance is running");
    
    assert!(request.status().is_success())
}