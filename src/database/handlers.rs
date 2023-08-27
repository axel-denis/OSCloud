use std::process::exit;

use bcrypt::{hash, DEFAULT_COST};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::database::model::{NewUser, Role, User};
use crate::database::schema::users::dsl::users;
use crate::database::schema::users::name;
use crate::database::Result;
use crate::database::{PostgresPool, UserData};

impl UserData {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: PostgresPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let dirs = directories::ProjectDirs::from("", "OsCloud", "oscloud")
            .expect("No project directory could be found on this OS, please report the bug as an issue on github: 'https://github.com/axel-denis/OSCloud-Back'");

        let mut path = dirs.config_dir().to_path_buf();
        path.push("./database/users.json");

        let data = UserData { pool, dirs };

        if path.exists() {
            if let Some(error) = data.import_default().err() {
                println!("import error: {error:?}");
                exit(0);
            }
        }
        data
    }
    /*
    pub fn users(&self) -> Vec<User> {
        self.get_users().unwrap()
    }
    */
    pub fn get_users(&self) -> Result<Vec<User>> {
        Ok(users.load::<User>(&mut self.pool.get()?)?)
    }
    /*
    pub fn delete_by_id(&self, id: i64) -> Result<()> {
        if users.find(id).count().first::<i64>(&mut self.pool.get()?)? <= 0 {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "User not found")));
        }
        diesel::delete(users.find(id)).execute(&mut self.pool.get()?)?;
        Ok(())
    }*/

    pub fn delete_user(&self, user_name: &str) -> Result<()> {
        if users
            .filter(name.eq(user_name))
            .count()
            .first::<i64>(&mut self.pool.get()?)?
            <= 0
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "User not found",
            )));
        }
        diesel::delete(users.filter(name.eq(user_name))).execute(&mut self.pool.get()?)?;
        Ok(())
    }

    pub fn create_user(&self, user_name: &str, user_password: &str, role: Role) -> Result<User> {
        if !users
            .filter(name.eq(user_name.clone()))
            .get_results::<User>(&mut self.pool.get()?)?
            .is_empty()
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "User already exist",
            )));
        }
        let hashed_password = hash(user_password, DEFAULT_COST)?;

        diesel::insert_into(users)
            .values(&NewUser {
                name: user_name.to_string(),
                password: hashed_password,
                user_role: role,
            })
            .execute(&mut self.pool.get()?)?;
        Ok(self.get_user_by_name(user_name).ok_or("Not Found")?)
    }

    pub fn get_user_by_id(&self, user_id: i64) -> Option<User> {
        users
            .find(user_id)
            .get_result::<User>(&mut self.pool.get().ok()?)
            .ok()
    }

    pub fn get_user_by_name(&self, user_name: &str) -> Option<User> {
        users
            .filter(name.eq(user_name))
            .first::<User>(&mut self.pool.get().ok()?)
            .ok()
    }
}
