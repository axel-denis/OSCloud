use std::path::PathBuf;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::database::model::{NewUser, User};
use crate::database::schema::users::dsl::users;
use crate::database::UserData;
use crate::database::Result;
use crate::database::schema::users::name;

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct ImportContent {
    users: Vec<NewUser>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct Content {
    users: Vec<User>,
}

impl UserData {
    pub fn save_default(&self) -> Result<()> {
        let mut path = self.dirs.config_dir().to_path_buf();

        path.push("./database/users.json");
        self.save_to_json(&path)
    }

    pub fn save_to_json(&self, path: &PathBuf) -> Result<()> {
        let content = Content { users: self.get_users()? };
        let serde_json = serde_json::to_string_pretty(&content)?;

        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p)?;
        }

        std::fs::write(path, serde_json.as_bytes())?;
        Ok(())
    }

    pub fn import_default(&self) -> Result<()> {
        let mut path = self.dirs.config_dir().to_path_buf();

        path.push("./database/users.json");
        self.import_from_json(&path)
    }

    pub fn import_from_json(&self, path: &PathBuf) -> Result<()> {
        let data = std::fs::read_to_string(path)?;
        let content: ImportContent = serde_json::from_str(&data)?;

        for user in content.users {
            match users.filter(name.eq(user.name.clone())).first::<User>(&mut self.pool.get()?) {
                Ok(_) => {continue;}
                Err(diesel::result::Error::NotFound) => {}
                Err(err) => { return Err(Box::new(err)) }
            }
            diesel::insert_into(users)
                .values(&user)
                .execute(&mut self.pool.get()?)?;
        }
        Ok(())
    }
}
