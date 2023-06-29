use serde::{Deserialize, Serialize};
use actix_web::http;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Admin,
    User
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub user_type: Type,
    #[serde(skip_serializing)]
    pub password: String,
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

pub fn get_user(username: &str, password: &str) -> Result<User, (http::StatusCode, std::io::Error)> {
    match get_users() {
        Ok(users) => {
            let found_user = users.iter().find(|&user| user.name == username);
            match found_user {
                Some(user) => {
                    if user.password == password {
                        Ok((*user).clone())
                    } else {
                        Err((http::StatusCode::UNAUTHORIZED, std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid password")))
                    }
                },
                None => Err((http::StatusCode::UNAUTHORIZED, std::io::Error::new(std::io::ErrorKind::NotFound, "User not found"))),
            }
        }
        Err(err) => Err((http::StatusCode::INTERNAL_SERVER_ERROR, err))
    }
}
