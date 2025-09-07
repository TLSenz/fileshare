
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{http, Error};
use axum::body::Body;
use axum::http::{HeaderValue, Response, StatusCode};
use axum::extract::{Request, State};
use axum::middleware::Next;
use dotenv::dotenv;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, Validation, decode, DecodingKey, TokenData, errors::Error as JwtError};
use sqlx::PgPool;
use crate::model::securitymodel::{AuthError, EncodeJWT};
use crate::model::securitymodel::AuthError::*;
use crate::model::usermodel::ConversionError;
use crate::repository::userrepository::check_if_user_exist;

pub fn encode_jwt(name: &str, email: &str) -> Result<String, ConversionError>{
    // Set token expiration to 24 hours from now
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize + 86400; // 24 hours in seconds

    let jwt_info = EncodeJWT {
        username: name.to_string(),
        email: email.to_string(),
        exp: expiration
    };

    dotenv().ok();
    let secret = env::var("JWT_SECRET")?;
    let token = encode(&Header::default(), &jwt_info, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|e| ConversionError::ConversionError(format!("JWT encoding error: {}", e)))?;
    Ok(token)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<EncodeJWT>, ConversionError> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET")?;

    // Create validation with expiration validation enabled
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let token_message = decode::<EncodeJWT>(
        &jwt_token, 
        &DecodingKey::from_secret(secret.as_ref()), 
        &validation
    ).map_err(|e| {
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => 
                ConversionError::ConversionError("Token has expired".to_string()),
            _ => ConversionError::ConversionError(format!("JWT decoding error: {}", e))
        }
    })?;

    Ok(token_message)
}

pub async fn authenticate(
    State(pool): State<PgPool>,
    mut req: Request,
    next: Next
) -> Result<Response<Body>, AuthError> {
    // Get authorization header
    let auth_header = req.headers().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| 
            AuthError("Invalid authorization header format".to_string(), StatusCode::FORBIDDEN)
        ),
        None => Err(AuthError("Authorization header is required".to_string(), StatusCode::FORBIDDEN))
    }?;

    // Split the header into parts and validate format
    let mut parts = auth_header.split_whitespace();
    let bearer = parts.next();
    let token = parts.next();

    // Validate bearer prefix
    if bearer != Some("Bearer") {
        return Err(AuthError("Invalid authorization scheme, expected 'Bearer'".to_string(), StatusCode::FORBIDDEN));
    }

    // Validate token presence
    let token = match token {
        Some(t) => t,
        None => return Err(AuthError("Token is missing".to_string(), StatusCode::FORBIDDEN))
    };

    // Decode and validate the token
    let token_data = decode_jwt(token.to_string())?;

    // Check if user exists in database
    match check_if_user_exist(pool, token_data.claims).await {
        Ok(_) => {}, // User exists, continue
        Err(_) => return Err(AuthError("User in JWT token does not exist in database".to_string(), StatusCode::FORBIDDEN))
    }

    Ok(next.run(req).await)
}
