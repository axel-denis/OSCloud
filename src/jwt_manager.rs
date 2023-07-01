use serde::{Deserialize, Serialize};
use crate::users::User;


#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    pub sub: i64,
    pub exp: usize,
}

fn get_secret() -> Vec<u8> {
    std::env::var("ACCESS_TOKEN_SECRET")
        .expect("ACCESS_TOKEN_SECRET must be set.")
        .into_bytes()
}

pub fn encode_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration_time = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(1800))
        .expect("invalid timestamp")
        .timestamp();
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
    let claims = Claims {
        sub: user.id,
        exp: expiration_time as usize,
    };
    let key = jsonwebtoken::EncodingKey::from_secret(&get_secret());
    let token = jsonwebtoken::encode(&header, &claims, &key)?;

    Ok(token)
}
