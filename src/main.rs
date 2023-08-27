mod auth_middleware;
#[cfg(feature = "cli")]
mod cli;
mod database;
mod jwt_manager;
mod network;
mod services;

use database::UserData;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    let userdata = UserData::new();

    #[cfg(feature = "cli")]
    cli::start_cli(&userdata);
    network::launch_actix(userdata).expect("actix launch crashed");
}
