pub(crate) mod debug;
pub(crate) mod handlers;
pub(crate) mod json;
pub(crate) mod model;
pub(crate) mod schema;


use diesel::{r2d2, PgConnection};
use directories::ProjectDirs;
use r2d2::ConnectionManager;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type PostgresPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct UserData {
    pool: PostgresPool,
    pub(crate) dirs: ProjectDirs,
}
