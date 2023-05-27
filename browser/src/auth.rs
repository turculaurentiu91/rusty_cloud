use interfaces::auth::{LoginRequest, LoginResponse, User};

pub async fn login(email: String, password: String) -> Result<String, String> {
    let request = LoginRequest { email, password };

    let client = reqwest::Client::new();

    let response = client
        .post("http://localhost:3000/api/auth/login")
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<LoginResponse>()
        .await
        .map_err(|e| e.to_string())?;

    match response {
        LoginResponse::Success(response) => Ok(response.user.email),
        LoginResponse::Error(error) => Err(error.error),
    }
}

pub async fn me() -> Option<User> {
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:3000/api/me")
        .send()
        .await
        .ok()?
        .json::<User>()
        .await
        .ok()?;

    Some(response)
}
