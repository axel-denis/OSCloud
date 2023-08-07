use colored::Colorize;

pub type HelpFn = fn() -> ();
pub type HelpMap = std::collections::HashMap<String, HelpFn>;

pub(crate) fn help_help() {
    println!("Usage: alias 'h'");
    println!("    help [command]");
    println!("Display usage of main program or the usage of the command given in input");
}

pub(crate) fn users_help() {
    println!("Usage: alias 'u'");
    println!("    users");
    println!("Display a formated table containing information about all the registered users");
}

pub(crate) fn exit_help() {
    println!("Usage: alias 'e', 'quit', 'q'");
    println!("    exit [option]");
    println!("Exit the program, and save the current database state to the default json folder {}", "'./database/users.json'".black());
    println!("");
    println!("    -n, --no-backup: disable the saving");
    println!("");
    println!("{}", "This keeps the default behaviour, each route thread have the default timout duration to finish their process".black())
}

pub(crate) fn clear_help() {
    println!("Usage: alias 'c'");
    println!("    clear");
    println!("Clear the current terminal");
    println!("{}", "Please report if machine does not support".black());
}

pub(crate) fn create_help_map() -> HelpMap {
    let mut map = HelpMap::new();

    map.insert("h".to_owned(), help_help);
    map.insert("help".to_owned(), help_help);
    map.insert("u".to_owned(), users_help);
    map.insert("users".to_owned(), users_help);
    map.insert("e".to_owned(), exit_help);
    map.insert("exit".to_owned(), exit_help);
    map.insert("q".to_owned(), exit_help);
    map.insert("quit".to_owned(), exit_help);
    map.insert("c".to_owned(), clear_help);
    map.insert("clear".to_owned(), clear_help);
    map
}

pub(crate) fn help(args: Vec<&str>, _: &crate::database::UserDatabase) {
    let map = create_help_map();
    if let Some(cmd) = args.get(1) {
        if let Some(function) = map.get(*cmd) {
            return function();
        } else {
            println!("Command selected don't exist");
        }
    }
    println!("Usage:");
    println!("    help, h\tDisplay this usage message");
    println!("    users, u\tDisplay the list of users in the database and all the data they hold");
    println!("    exit, quit, e, q\tClose the program and save the current data base");
    println!("    clear, c\tClear the current terminal");
    println!("");
    println!("See 'help <command>' for more information on a specific command.");
}
