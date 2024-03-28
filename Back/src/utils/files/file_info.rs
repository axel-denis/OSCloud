use std::fs::metadata;

pub fn check_path_is_folder(path: &String) -> bool {
    match metadata(path) {
        Ok(md) => md.is_dir(),
        Err(_) => false,
    }
}

// pub fn check_path_is_file(path: &String) -> bool {
//     match metadata(path) {
//         Ok(md) => md.is_file(),
//         Err(_) => false,
//     }
// }
