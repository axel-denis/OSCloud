use std::path::PathBuf;
use path_clean::clean;

pub fn clean_path(path: &String) -> PathBuf {
    let mut selection: &str = &*path;
    while selection.starts_with(".") {
        selection = &selection[1..];
    }
    clean(selection)
}
