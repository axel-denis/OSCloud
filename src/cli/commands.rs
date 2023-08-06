use std::collections::HashMap;

use crate::database::UserDatabase;

pub type CommandsFn = fn(Vec<String>, &UserDatabase) -> ();

pub type CommandsMap = HashMap<String, CommandsFn>;

pub(crate) fn create_commands_map() -> CommandsMap {
    let mut map = CommandsMap::new();

    map.insert("h".to_owned(), help);
    map.insert("help".to_owned(), help);
    map.insert("u".to_owned(), debug_users);
    map.insert("users".to_owned(), debug_users);
    map.insert("e".to_owned(), exit);
    map.insert("exit".to_owned(), exit);
    map
}

pub(crate) fn debug_users(_: Vec<String>, db: &UserDatabase) {
    db.pretty_print()
}

pub(crate) fn help(_: Vec<String>, _: &UserDatabase) {
    println!("Usage:");
    println!("    help, h\tDisplay this usage message");
    println!("    users, u\tDisplay the list of users in the database and all the data they hold\n");
    println!("    exit, e\tClose the program and save the current data base");
    print!("See 'help <command>' for more information on a specific command.\n");
}

pub(crate) fn exit(_: Vec<String>, db: &UserDatabase) {
    if let Err(err) = db.save_to_json() {
        println!("{err}");
    } else {
        std::process::exit(0);
    }
}
