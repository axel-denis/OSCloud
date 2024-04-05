use std::{fs, path::PathBuf};

use crate::database::{
    model::{ShareType, User},
    UserData,
};

pub struct VerifiedUserPath {
    _user: User,
    _path: PathBuf,
}

impl VerifiedUserPath {
    pub fn new(user: User, path: PathBuf) -> VerifiedUserPath {
        VerifiedUserPath {
            _user: user,
            _path: path,
        }
    }
    pub fn user(&self) -> &User {
        &self._user
    }
    pub fn path(&self) -> &PathBuf {
        &self._path
    }
}

pub fn verifiy_user_path(db: &UserData, path: &String, user: User) -> Option<VerifiedUserPath> {
    let mnts = db.get_user_mounts_points(&user)?;
    let canonical = match fs::canonicalize(path) {
        Ok(result) => result,
        Err(_) => return None,
    };
    for mnt in mnts {
        for ancestor in canonical.ancestors() {
            if ancestor == PathBuf::from(&mnt) {
                return Some(VerifiedUserPath::new(user, canonical));
            };
        }
    }
    None
}

// returns shared info if the provided user has access
// Note : goes up the path tree to find if the file is in a shared folder
pub fn verify_user_shared_path(
    db: &UserData,
    path: &String,
    user: User,
) -> Option<VerifiedUserPath> {
    let canonical = match std::fs::canonicalize(PathBuf::from(path)) {
        Ok(result) => result,
        Err(_) => return None,
    };
    for ancestor in canonical.ancestors() {
        if let Some(shares) = db.get_share_from_file_path(ancestor) {
            for share in shares {
                if share.share_type == ShareType::Public {
                    return Some(VerifiedUserPath::new(user, canonical));
                } else if let Some(users_shared_to) = db.get_file_users_shared_to(&share) {
                    if users_shared_to.contains(&user.id) {
                        return Some(VerifiedUserPath::new(user, canonical));
                    }
                }
            }
        };
    }
    None
}
