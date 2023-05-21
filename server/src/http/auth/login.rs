use axum::response::Json;
use interfaces::auth::{LoginRequest, LoginResponse};

use crate::{helpers::user::User, http::error::Error};

pub async fn handle(body: Json<LoginRequest>) -> Result<Json<LoginResponse>, Error<String>> {
    let token = User::new(1, "test", &body.email).to_oken();

    Ok(Json(LoginResponse { token }))
}
