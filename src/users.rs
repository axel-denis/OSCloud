use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct Content {
    users: Vec<User>
}

pub fn get_users() -> Result<Vec<User>, std::io::Error> {
    let mut users_file = File::open("database/users.json")?;
    let mut data = String::new();
    users_file.read_to_string(&mut data)?;
    let content: Content = serde_json::from_str(&data)?;
    Ok(content.users)
}
