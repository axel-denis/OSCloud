use std::fs;
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
use crate::utils::files::file_info::check_path_is_folder;

use super::model::{NewUserMountPoint, UserMountPoint};

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

    pub fn create_user(&self, user_name: &str, user_password: &str, role: Role, enable: bool) -> Result<User> {
        let mut pool = self.pool.get()?;
        if !users
            .filter(name.eq(user_name))
            .get_results::<User>(&mut pool)?
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
                enabled: enable,
            })
            .execute(&mut pool)?;
        Ok(self.get_user_by_name(user_name).ok_or("Not Found")?)
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Option<User> {
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

    // pub fn get_user_mount_point_by_id(&self, id: i32) -> Option<UserMountPoint> {
    //     crate::database::schema::user_mounts_points::dsl::user_mounts_points
    //         .find(id)
    //         .get_result::<UserMountPoint>(&mut self.pool.get().ok()?)
    //         .ok()
    // }

    pub fn add_user_mount_point(&self, user: &User, path: &String) -> Result<UserMountPoint> {
        let pathed_path = match fs::canonicalize(path)?.into_os_string().into_string() {
            Ok(path) => path,
            Err(_) => return Err("Mount point could not be resolved into a string".into()),
        };
        if !check_path_is_folder(&path) {
            return Err("Mount point need to be a valid folder".into());
        }
        let mut pool = self.pool.get()?;
        Ok(diesel::insert_into(
            crate::database::schema::user_mounts_points::dsl::user_mounts_points,
        )
        .values(&NewUserMountPoint {
            user_id: user.id,
            path: pathed_path,
        })
        .get_result::<UserMountPoint>(&mut pool)?)
    }

    pub fn get_user_mounts_points(&self, user: &User) -> Option<Vec<String>> {
        let pool = &mut self.pool.get().ok()?;
        let test: Vec<String> = UserMountPoint::belonging_to(&user)
            .select(UserMountPoint::as_select())
            .load::<UserMountPoint>(pool)
            .ok()?
            .iter()
            .map(|ump| ump.path.clone())
            .collect();
        Some(test)
    }
}
