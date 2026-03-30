use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub identifier: String,
    pub username: String,
    pub password: String,
    pub geo_location: Option<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub flow_id: String,
    pub user_id: String,
    pub identifier: String,
    pub session_cookie: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub identifier: String,
    pub session_cookie: String,
}

#[derive(Deserialize)]
pub struct RecoveryRequestDto {
    pub email: String,
}

#[derive(Deserialize)]
pub struct SendCodeRequestDto {
    pub email: String,
}

#[derive(Deserialize)]
pub struct SubmitCodeRequestDto {
    pub code: String,
}

#[derive(Deserialize)]
pub struct VerifyByLinkRequestDto {
    pub email: String,
}

#[derive(Deserialize)]
pub struct SettingsRequestDto {
    pub method: String,
    pub password: Option<String>,
    pub traits: Option<serde_json::Value>,
    pub lookup_secret_confirm: Option<bool>,
    pub lookup_secret_disable: Option<bool>,
    pub lookup_secret_regenerate: Option<bool>,
    pub lookup_secret_reveal: Option<bool>,
    pub transient_payload: Option<serde_json::Value>,
}
