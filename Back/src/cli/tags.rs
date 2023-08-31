use crate::database::UserData;

use super::{commands::CmdStatus, formating::{err_str, ok_str}};


pub(crate) fn debug_tags(_: Vec<&str>, db: &UserData) -> CmdStatus {
    match db.pretty_format_tags() {
        Ok(str) => println!("{str}"),
        Err(err) => println!("{}", err_str(err)),
    }
    CmdStatus::Ok
}

pub(crate) fn create_tag(args: Vec<&str>, db: &UserData) -> CmdStatus {
    if args.len() != 2 {
        println!(
            "{} create_tag <name> {} help 'create_tag'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        return CmdStatus::Ok;
    }

    match db.create_tag(args[1]) {
        Ok(tag) => println!("Tag {} created!", ok_str(tag.name)),
        Err(err) => println!("{}", err_str(err)),
    }
    CmdStatus::Ok
}
