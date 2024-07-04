use serde::{Deserialize, Serialize};

pub mod handler;
pub mod jwt;

#[derive(Debug, Deserialize, Clone)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

// In-memory storage
#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub users: std::sync::Mutex<std::collections::HashMap<String, String>>,
}
