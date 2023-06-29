mod users;
use serde::{Deserialize, Serialize};
use users::User;
use dotenv::dotenv;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct Network {
    users: Vec<User>
}

fn main() {
    dotenv().ok();
    let mut users_file = File::open("database/users.json").unwrap();
    let mut data = String::new();
    users_file.read_to_string(&mut data).unwrap();

    let network: Network = serde_json::from_str(&data).unwrap();
    let token = std::env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set.");

    println!("{:?} {:?}", network, token);
}
