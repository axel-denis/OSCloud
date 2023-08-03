mod auth_middleware;
mod jwt_manager;
mod network;
mod services;
mod users;
mod register;
mod database;

use database::UserDatabase;
use dotenv::dotenv;
fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    let userbase = UserDatabase::new();
    if let Some(error) = userbase.import_from_json().err() {
        println!("error: {error:?}");
        return;
    }
    userbase.pretty_print();
    return;

    network::launch_actix(userbase).expect("actix launch crashed");
}
