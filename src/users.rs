use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Admin,
    User
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: i64,
    name: String,
    #[serde(rename = "type")]
    user_type: Type,
    password: String,
}
