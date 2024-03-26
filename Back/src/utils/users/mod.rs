use std::{fs, path::PathBuf};

use crate::database::{model::User, UserData};

pub struct VerifiedUserPath {
    _user: User,
    _path: PathBuf,
}

impl VerifiedUserPath {
    pub fn user(&self) -> &User {
        &self._user
    }
    pub fn path(&self) -> &PathBuf {
        &self._path
    }
}

pub fn path_in_user_mounts_points(
    db: &UserData,
    path: &String,
    user: User,
) -> Option<VerifiedUserPath> {
    let mnts = db.get_user_mounts_points(&user)?;
    for mnt in mnts {
        if path.starts_with(&mnt) {
            return match fs::canonicalize(mnt) {
                Ok(result) => Some(VerifiedUserPath {
                    _user: user,
                    _path: result,
                }),
                Err(_) => None,
            };
        }
    }
    None
}
