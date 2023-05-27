use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};
use hmac::{Hmac, Mac};
use interfaces::error_response::ErrorResponse;
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;
use tower_cookies::Cookies;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}

pub enum TokenError {
    Inernal(String),
    Decode(jwt::Error),
}

impl From<jwt::Error> for TokenError {
    fn from(value: jwt::Error) -> Self {
        Self::Decode(value)
    }
}

impl User {
    pub fn new(id: i32, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    pub fn to_token(&self) -> String {
        let secret = env::var("APP_SECRET").expect("APP_SECRET must be set");
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(secret.as_bytes()).expect("Could not create HMAC key");
        self.sign_with_key(&key).unwrap()
    }

    pub fn from_token(s: &str) -> Result<Self, TokenError> {
        let secret = env::var("APP_SECRET")
            .map_err(|_| TokenError::Inernal(String::from("APP_SECRET must be set")))?;
        let key: Hmac<Sha256> =
            Hmac::new_from_slice(secret.as_bytes()).expect("Could not create HMAC key");
        s.verify_with_key(&key).map_err(jwt::Error::into)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ErrorResponse<String>>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|e| {
                (
                    e.0,
                    Json(ErrorResponse {
                        error: e.1.to_string(),
                        extra: None,
                    }),
                )
            })?;

        if let Some(session) = cookies.get("session") {
            let token = session.value().trim();
            return User::from_token(token).map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        error: "Invalid session".to_string(),
                        extra: None,
                    }),
                )
            });
        }

        Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid session".to_string(),
                extra: None,
            }),
        ))
    }
}
