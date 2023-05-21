use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;

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

    pub fn toToken(&self) -> String {
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
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(authorization) = parts.headers.get(AUTHORIZATION) {
            if let Ok(authorization) = authorization.to_str() {
                if let Some(token) = authorization.split_once(" ") {
                    return User::from_token(token.1.trim()).map_err(|_| StatusCode::UNAUTHORIZED);
                }
            }
        }

        Err(StatusCode::UNAUTHORIZED)
    }
}
