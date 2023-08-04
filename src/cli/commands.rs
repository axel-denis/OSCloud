use std::collections::HashMap;

use crate::database::UserDatabase;

pub type CommandsFn = fn(&UserDatabase) -> ();

pub type CommandsMap = HashMap<String, CommandsFn>;

pub(crate) fn create_commands_map() -> CommandsMap {
    let mut map = CommandsMap::new();

    map.insert("help".to_owned(), help);
    map
}


pub(crate) fn help(db: &UserDatabase) {
    println!("So you need help hun")
}
