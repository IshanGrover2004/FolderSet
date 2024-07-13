use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bcrypt::hash;
use diesel::prelude::*;
use crate::{database::schema::users, SignupRequest};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn from_signup_request(request: &SignupRequest) -> Result<Self, bcrypt::BcryptError> {
        let hashed_password = hash(&request.password, 7)?;
        Ok(NewUser {
            id: Uuid::new_v4(),  // Generate a new UUID
            name: request.name.clone(),
            email: request.email.clone(),
            password: hashed_password,
        })
    }
}