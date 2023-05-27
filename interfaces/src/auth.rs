use serde::{Deserialize, Serialize};

use crate::error_response::ErrorResponse;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginOkResponse {
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub enum LoginResponse {
    Success(LoginOkResponse),
    Error(ErrorResponse<String>),
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum RegisterResponse {
    Success(User),
    Error(ErrorResponse<String>),
}
