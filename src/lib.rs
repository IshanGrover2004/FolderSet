use serde::{Deserialize, Serialize};


pub mod database;
pub mod handler;
pub mod jwt;


#[derive(Debug, Deserialize, Serialize ,Clone)]
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
