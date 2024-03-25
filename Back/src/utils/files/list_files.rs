use core::fmt;
use std::{ffi::OsString, fs::read_dir};

pub enum FileType {
    File,
    Folder,
    Other,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::File => write!(f, "File"),
            FileType::Folder => write!(f, "Folder"),
            FileType::Other => write!(f, "Other"),
        }
    }
}

pub struct FileInfo {
    pub name: OsString,
    pub file_type: FileType,
}

pub fn list_files(path: String) -> Result<Vec<FileInfo>, std::io::Error> {
    let paths = read_dir(path)?;

    Ok(paths
        .flatten()
        .map(|pth| FileInfo {
            name: pth.file_name(),
            file_type: if pth.path().is_file() {
                FileType::File
            } else if pth.path().is_dir() {
                FileType::Folder
            } else {
                FileType::Other
            },
        })
        .collect())
}
