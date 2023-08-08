mod auth_middleware;
mod jwt_manager;
mod network;
mod services;
mod database;
mod cli;

use database::UserDatabase;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    let userbase = UserDatabase::new();
    if std::path::Path::new("database/users.json").exists() {
        if let Some(error) = userbase.import_default().err() {
            println!("import error: {error:?}");
            return;
        }
    }
    cli::start_cli(&userbase);
    network::launch_actix(userbase).expect("actix launch crashed");
}
