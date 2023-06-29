mod users;
mod services;
mod network;

use users::get_users;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let token = std::env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set.");

    match get_users() {
        Err(err) => {
            println!("Unable to obtain users: {err}");
        }
        Ok(users) => {
            println!("{:?} {:?}", users, token);
            network::launch_actix().unwrap();
        }
    }
}
