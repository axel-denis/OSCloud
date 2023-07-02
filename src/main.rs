mod auth_middleware;
mod jwt_manager;
mod network;
mod services;
mod users;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");
    network::launch_actix().expect("actix launch crashed");
}
