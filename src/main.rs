mod auth_middleware;
mod jwt_manager;
mod network;
mod services;
mod users;

use dotenv::dotenv;
use users::get_users_from_database;

fn main() {
    dotenv().ok();

    let token = std::env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set.");

    std::env::set_var("RUST_LOG", "actix_web=debug");
    match get_users_from_database() {
        Err(err) => {
            println!("Unable to obtain users: {err}");
        }
        Ok(users) => {
            println!("{users:?} {token:?}");
            network::launch_actix().expect("actix launch crashed");
        }
    }
}
