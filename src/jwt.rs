use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

const SECRET_KEY: &[u8] = b"makerstudio"; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
