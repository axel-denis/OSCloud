mod auth_middleware;
mod jwt_manager;
mod network;
mod services;
mod database;
mod cli;

use database::UserData;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    let userdata = UserData::new();

    cli::start_cli(&userdata);
    network::launch_actix(userdata).expect("actix launch crashed");
}
