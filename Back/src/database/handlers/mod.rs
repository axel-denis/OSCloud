use std::process::exit;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::database::{PostgresPool, UserData};

pub mod user_advanced;
pub mod user_basis;
pub mod file_shares;

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
}
