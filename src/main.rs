mod users;
use serde::{Deserialize, Serialize};
use users::User;

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct Network {
    users: Vec<User>
}

fn main() {
    let json = r#"
    [
      {
        "id": 0,
        "name": "Axel",
        "type": "admin",
        "password": "password"
      },
      {
        "id": 1,
        "name": "Arthur",
        "type": "user",
        "password": "mdp"
      },
      {
        "id": 2,
        "name": "test",
        "type": "user",
        "password": "test"
      }
    ]
    "#;

    let network: Network = serde_json::from_str(json).unwrap();
    println!("{:?}", network);
}
