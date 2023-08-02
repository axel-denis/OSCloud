use serde::{Deserialize, Serialize};
use crate::users::Type;
use std::{io::Error, fs};
use itertools::Itertools;
use bcrypt::{DEFAULT_COST, hash};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct ContentFullySerialized {
    users: Vec<UserFullySerialized>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserFullySerialized {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub user_type: Type,
    pub password: String,
}

pub fn register_new_user_in_database(name: &str, password: &str, user_type: Type) -> Result<(), std::io::Error> {
    let data = fs::read_to_string("database/users.json")?;

    let mut content: ContentFullySerialized = serde_json::from_str(&data)?;

    if content.users.iter().any(|user| user.name == name) {
        return Err(Error::new(std::io::ErrorKind::NotFound, "Username already taken"));
    }
    let mut id = content.users.iter().map(|user| user.id).collect::<Vec<i64>>();
    id.sort();
    let mut new_id = id.last().unwrap_or(&(-1)) + 1;
    for i in &id {
        println!("{i}");
    }
    for (fst, snd) in id.iter().tuple_windows() {
        if *snd != fst + 1 {
            new_id = fst + 1;
            break;
        }
    }
    match hash(&password, DEFAULT_COST) {
        Ok(hash) => {
            println!("{new_id}");
            let new_user = UserFullySerialized {
                name: name.to_string(),
                password: hash,
                id: new_id,
                user_type,
            };
            content.users.push(new_user);
            let serde_json = serde_json::to_string_pretty(&content)?;
            fs::write("database/users.json", serde_json.as_bytes())?;
            Ok(())
        },
        Err(err) => return Err(Error::new(std::io::ErrorKind::InvalidData, err.to_string())),
    }
}
