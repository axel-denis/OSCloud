use std::{
    ffi::OsString,
    path::{Component, PathBuf},
};

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
    let entire_path = clean_path(path);
    for mnt in mnts {
        for ancestor in entire_path.ancestors() {
            if ancestor == PathBuf::from(&mnt) {
                return Some(VerifiedUserPath {
                    user,
                    path: entire_path,
                });
            };
        }
    }
    None
}

// as it is right now, path should not begin with /
fn change_path_origin(origin: String, path: PathBuf) -> PathBuf {
    let mut elements: Vec<_> = path.components().collect();
    let new_origin = OsString::from(origin);
    elements[0] = Component::Normal(&new_origin);
    elements.iter().collect()
}

// TODO - test the whole function
// returns shared info if the provided user has access
// as it is right now, path should not begin with / but with the share code // TODO - process if starts with /
pub fn verify_user_shared_path(
    db: &UserData,
    path: &String,
    user: User,
) -> Option<VerifiedUserPath> {
    let entire_path = clean_path(path);
    let code = entire_path
        .ancestors()
        .last()?
        .to_string_lossy()
        .to_string(); // TODO - check if a / is included
    if let Some(share) = db.get_share_from_code(&code) {
        let new_path = change_path_origin(share.path, entire_path);
        if new_path.exists() {
            if new_path.is_dir() {
                match share.share_type {
                    ShareType::Public => Some(VerifiedUserPath::new(user, new_path)),
                    ShareType::User => Some(VerifiedUserPath::new(user, new_path)), // TODO
                }
            } else {
                None // TODO (gives access if only the code is given. If path after code, refuse or only give file pointed by the code)
            }
        } else {
            None
        }
    } else {
        None
    }
    // for ancestor in clean_path.ancestors() {
    //     if let Some(shares) = db.get_share_from_file_path(ancestor) {
    //         for share in shares {
    //             if share.share_type == ShareType::Public {
    //                 return Some(VerifiedUserPath::new(user, clean_path));
    //             } else if let Some(users_shared_to) = db.get_file_users_shared_to(&share) {
    //                 if users_shared_to.contains(&user.id) {
    //                     return Some(VerifiedUserPath::new(user, clean_path));
    //                 }
    //             }
    //         }
    //     };
    // }
    // None
}
