mod commands;
mod help;
mod users;
mod formating;

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use crate::cli::commands::CmdStatus;

use std::thread;
use colored::Colorize;

use crate::cli::commands::CommandsMap;
use crate::cli::formating::err_str;
use crate::database::UserData;

pub fn execute_command(db: &UserData, line: String, map: &CommandsMap) -> CmdStatus {
    let parts: Vec<&str> = line.split(" ").collect();
    match map.get(parts[0]) {
        Some(commands) => commands(parts, &db),
        None => {println!("Unrecognized cmd!"); CmdStatus::Ok},
    }
}

pub fn start_cli(db: &UserData) {
    let db = db.clone();
    thread::spawn(move || {
        println!("Write {} to get the different commands", "help".bold());

        let mut rl = DefaultEditor::new().unwrap();
        let mut have_to_save = true;
        let mut path = db.dirs.config_dir().to_path_buf();
        path.push("history");
        rl.load_history(&path).unwrap();

        let map: CommandsMap = commands::create_commands_map();
        loop {
            let readline = rl.readline(format!("{} #$ ", "OsCloud".cyan()).as_str());
            match readline {
                Ok(line) => {
                    if let Err(err) = rl.add_history_entry(line.as_str()) {
                        println!("{}", err_str(err.to_string()));
                    }
                    let status = execute_command(&db, line, &map);
                    if status == CmdStatus::Exit {
                        break
                        have_to_save = false;
                    }
                    if status == CmdStatus::ExitWithBackup {
                        break
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
        if let Err(err) = rl.save_history(&path) {
            println!("{}", err_str(err.to_string()));
        }
        println!("Exiting...");
        if have_to_save {
            crate::cli::users::save(vec!["save"], &db);
        }
        std::process::exit(0);
    });
}
