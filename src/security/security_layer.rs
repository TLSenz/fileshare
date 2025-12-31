use crate::model::RateError;
use crate::model::securitymodel::AuthError::*;
use crate::model::securitymodel::{AuthError, EncodeJWT};
use crate::model::usermodel::ConversionError;
use crate::repository::userrepository::check_if_user_exist;
use axum::body::Body;
use axum::extract::{ConnectInfo, Request, State};
use axum::http;
use axum::http::{HeaderMap, Response, StatusCode};
use axum::middleware::Next;
use dotenv::dotenv;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use redis::AsyncCommands;
use sqlx::PgPool;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn encode_jwt(name: &str, email: &str) -> Result<String, ConversionError> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
        + 86400;
    let jwt_info = EncodeJWT {
        username: name.to_string(),
        email: email.to_string(),
        exp: expiration,
    };

    dotenv().ok();
    let secret = env::var("JWT_SECRET")?;
    let token = encode(
        &Header::default(),
        &jwt_info,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| ConversionError::ConversionError(format!("JWT encoding error: {}", e)))?;
    Ok(token)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<EncodeJWT>, ConversionError> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET")?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let token_message = decode::<EncodeJWT>(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            ConversionError::ConversionError("Token has expired".to_string())
        }
        _ => ConversionError::ConversionError(format!("JWT decoding error: {}", e)),
    })?;

    Ok(token_message)
}

pub async fn authenticate(
    State(pool): State<PgPool>,
    req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| {
            AuthError(
                "Invalid authorization header format".to_string(),
                StatusCode::FORBIDDEN,
            )
        }),
        None => Err(AuthError(
            "Authorization header is required".to_string(),
            StatusCode::FORBIDDEN,
        )),
    }?;

    let mut parts = auth_header.split_whitespace();
    let bearer = parts.next();
    let token = parts.next();

    // Validate bearer prefix
    if bearer != Some("Bearer") {
        return Err(AuthError(
            "Invalid authorization scheme, expected 'Bearer'".to_string(),
            StatusCode::FORBIDDEN,
        ));
    }

    let token = match token {
        Some(t) => t,
        None => {
            return Err(AuthError(
                "Token is missing".to_string(),
                StatusCode::FORBIDDEN,
            ));
        }
    };

    let token_data = decode_jwt(token.to_string())?;

    match check_if_user_exist(pool, token_data.claims).await {
        Ok(_) => {} // User exists, continue
        Err(_) => {
            return Err(AuthError(
                "User in JWT token does not exist in database".to_string(),
                StatusCode::FORBIDDEN,
            ));
        }
    }

    Ok(next.run(req).await)
}

// Improve Logging with getting the User Info from the JWT
pub async fn rate_limit(
    request: Request,
    next: Next,
) -> Result<Response<Body>, RateError<'static>> {
    let mut r = match redis::Client::open("redis://127.0.0.1") {
        Ok(client) => match client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(_e) => {
                tracing::error!("Could not connect to Redis; letting request pass");
                return Ok(next.run(request).await);
            }
        },
        Err(e) => {
            println!("Could not parse Redis URL: {e}");
            return Ok(next.run(request).await);
        }
    };
    let client_rate_limit = 100;
    let ttl = 1;

    if let Some(ip_address) = request.extensions().get::<ConnectInfo<SocketAddr>>() {
        tracing::debug!("Client IP {:?}", ip_address);
        let count: i32 = AsyncCommands::incr(&mut r, ip_address.to_string(), 1).await?;
        if count == 1 {
            let _: () = AsyncCommands::expire(
                &mut r,
                ip_address.to_string(),
                (ttl * 60).try_into().unwrap(),
            )
            .await?;
        }
        if count > client_rate_limit {
            tracing::warn!("Client exided Rate Limit");
            return Err(RateError::RateError(
                "Rate limit exceeded",
                StatusCode::TOO_MANY_REQUESTS,
            ));
        }
        return Ok(next.run(request).await);
    }
    return Err(RateError::RateError(
        "Rate limit exceeded",
        StatusCode::TOO_MANY_REQUESTS,
    ));
}

fn get_ip(header: &HeaderMap) -> Option<IpAddr> {
    if let Some(client_ip_adress) = header.get("X-Forwarded-For") {
        if let Ok(client_ip_adress) = client_ip_adress.to_str() {
            if let Some(client_ip) = client_ip_adress.split(",").next() {
                if let Ok(ip) = client_ip.trim().parse() {
                    return Some(ip);
                }
            }
        }
    }
    None
}
