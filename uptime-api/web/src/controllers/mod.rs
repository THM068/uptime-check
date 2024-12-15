pub mod users;
pub mod authentication;
pub mod error;
use bcrypt::{BcryptResult, verify};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ReqSignIn {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResSignIn {
    pub token: String,
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap()
}
