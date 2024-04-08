use std::path::PathBuf;

use crate::database::{
    model::{ShareType, User},
    UserData,
};

use super::files::clean_path::clean_path;

#[must_use]
pub struct VerifiedUserPath {
    user: User,
    path: PathBuf,
}

impl VerifiedUserPath {
    pub fn new(user: User, path: PathBuf) -> VerifiedUserPath {
        VerifiedUserPath {
            user: user,
            path: path,
        }
    }
    pub fn user(&self) -> &User {
        &self.user
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

pub fn verifiy_user_path(db: &UserData, path: &String, user: User) -> Option<VerifiedUserPath> {
    let mnts = db.get_user_mounts_points(&user)?;
    let clean_path = clean_path(path);
    for mnt in mnts {
        for ancestor in clean_path.ancestors() {
            if ancestor == PathBuf::from(&mnt) {
                return Some(VerifiedUserPath {
                    user,
                    path: clean_path,
                });
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
