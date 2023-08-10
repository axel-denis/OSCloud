use crate::cli::formating::info_str;
use crate::database::UserData;
use crate::cli::commands::CmdStatus;

pub type HelpFn = fn(&UserData) -> ();
pub type HelpMap = std::collections::HashMap<String, HelpFn>;

pub(crate) fn help_help(_: &UserData) {
    println!("Usage: alias 'h'");
    println!("    help [command]");
    println!("Display usage of main program or the usage of the command given in input");
}

pub(crate) fn users_help(_: &UserData) {
    println!("Usage: alias 'u'");
    println!("    users");
    println!("Display a formatted table containing information about all the registered users");
}

pub(crate) fn exit_help(_: &UserData) {
    println!("Usage: alias 'e', 'quit', 'q'");
    println!("    exit [option]");
    println!("Exit the program, and save the current database state to the default json folder {}", info_str("'./database/users.json'"));
    println!();
    println!("    -n, --no-backup: disable the saving");
    println!();
    println!("{}", info_str("This keeps the default behaviour, each route thread have the default timout duration to finish their process"))
}

pub(crate) fn clear_help(_: &UserData) {
    println!("Usage: alias 'c'");
    println!("    clear");
    println!("Clear the current terminal");
    println!("{}", info_str("Please report if machine does not support"));
}

pub(crate) fn delete_user_help(_: &UserData) {
    println!("Usage: alias 'du'");
    println!("    delete_user <username>");
    println!("Delete the user from the database that matches the username");
}

pub(crate) fn create_user_help(_: &UserData) {
    println!("Usage: alias 'cu'");
    println!("    create_user <unique_username> <secure_password> <(Admin|User)>");
    println!("Create an new user in the database using the selected credentials");
}

pub(crate) fn save_help(db: &UserData) {
    println!("Usage: alias 's'");
    println!("    save [path]");
    println!("Save the user database content to selected file");
    let mut path = db.dirs.config_dir().to_path_buf();

    path.push("database/users.json");
    let str = info_str(format!("Default path: '{path:?}'"));
    println!("{str}");
    println!("{}", info_str("Save as users.json if the file name is not specified"));
}

pub(crate) fn import_help(db: &UserData) {
    println!("Usage: alias 'i'");
    println!("    import [path]");
    println!("Import the user database from the selected file");
    let mut path = db.dirs.config_dir().to_path_buf();

    path.push("database/users.json");
    let str = info_str(format!("Default path: '{path:?}'"));
    println!("{str}");
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
    map.insert("s".to_owned(), save_help);
    map.insert("save".to_owned(), save_help);
    map.insert("i".to_owned(), import_help);
    map.insert("import".to_owned(), import_help);
    map.insert("cu".to_owned(), create_user_help);
    map.insert("create_user".to_owned(), create_user_help);
    map.insert("du".to_owned(), delete_user_help);
    map.insert("delete_user".to_owned(), delete_user_help);
    map
}

pub(crate) fn help(args: Vec<&str>, db: &crate::database::UserData) -> CmdStatus {
    let map = create_help_map();
    if let Some(cmd) = args.get(1) {
        if let Some(function) = map.get(*cmd) {
            function(db);
            return CmdStatus::Ok;
        } else {
            println!("Command selected don't exist");
        }
    }
    println!("Usage:");
    println!("    help, h\tDisplay this usage message");
    println!("    users, u\tDisplay the list of users in the database and all the data they hold");
    println!("    exit, quit, e, q\tClose the program and save the current data base");
    println!("    clear, c\tClear the current terminal");
    println!("    create_user, cu\tCreate new user");
    println!("    save, s\tSave database to file");
    println!("    import, i\tImport database from file");
    println!("    delete_user, du\tDelete user");
    println!();
    println!("See 'help <command>' for more information on a specific command.");
    CmdStatus::Ok
}
