use std::collections::HashMap;

use crate::database::UserDatabase;

pub type CommandsFn = fn(&UserDatabase) -> ();

pub type CommandsMap = HashMap<String, CommandsFn>;

pub(crate) fn create_commands_map() -> CommandsMap {
    let mut map = CommandsMap::new();

    map.insert("help".to_owned(), help);
    map.insert("users".to_owned(), debug_users);
    map
}

pub(crate) fn debug_users(db: &UserDatabase) {
    db.pretty_print()
}

pub(crate) fn help(_: &UserDatabase) {
    println!("Help:");
    println!("help: display the help");
    println!("users: display the list of users")
}
