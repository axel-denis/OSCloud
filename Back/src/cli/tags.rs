use crate::database::UserData;

use super::{commands::CmdStatus, formating::err_str};


pub(crate) fn debug_tags(_: Vec<&str>, db: &UserData) -> CmdStatus {
    match db.pretty_format_tags() {
        Ok(str) => println!("{str}"),
        Err(err) => println!("{}", err_str(err)),
    }
    CmdStatus::Ok
}
