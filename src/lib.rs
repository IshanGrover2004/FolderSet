use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::PgPool;

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


// User Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}


pub struct AppState {
    pub pool: PgPool,
}
