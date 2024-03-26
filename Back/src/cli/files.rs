use crate::cli::commands::CmdStatus;
use crate::cli::formating::err_str;
use crate::database::UserData;
use crate::utils::files;
use crate::utils::users::path_in_user_mounts_points;

pub(crate) fn list_files_as(args: Vec<&str>, db: &UserData) -> CmdStatus {
    if args.len() != 2 {
        println!(
            "{} list_files <path>{} help 'list_files'",
            err_str("Invalid arguments given, should be"),
            err_str(", for more informations try")
        );
        CmdStatus::Ok
    } else {
        let user = match db.get_user_by_name(args[1]) {
            Some(user) => user,
            None => {
                println!("Error user not found");
                return CmdStatus::Ok;
            }
        };
        let user_path = match path_in_user_mounts_points(db, &args[2].to_owned(), user) {
            Some(pth) => pth,
            None => {
                println!("Path not in user mounts points");
                return CmdStatus::Ok;
            }
        };
        let files = match files::list_files::list_files(&user_path) {
            Ok(files) => files,
            Err(e) => {
                println!("Error while listing files : {}", err_str(e));
                return CmdStatus::Ok;
            }
        };
        for file in files {
            println!(
                "{}:\t {}",
                file.file_type,
                file.name
                    .into_string()
                    .unwrap_or("### non unicode data ###".to_string())
            );
        }
        CmdStatus::Ok
    }
}
