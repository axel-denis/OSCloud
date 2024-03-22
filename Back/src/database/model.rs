use diesel::associations::{Associations, Identifiable};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug, PartialEq, Default, diesel_derive_enum::DbEnum, Clone, Deserialize, Serialize)]
#[ExistingTypePath = "crate::database::schema::sql_types::Role"]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    #[default]
    User,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => {
                write!(f, "Admin")
            }
            Role::User => {
                write!(f, "User")
            }
        }
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(input: &str) -> Result<Role, Self::Err> {
        match &*input.to_lowercase() {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::database::schema::users)]
pub struct NewUser {
    pub name: String,
    pub password: String,
    #[serde(rename = "type")]
    pub user_role: Role,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::database::schema::users)]
pub struct ShareableUser {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub user_role: Role,
    pub enabled: bool,
}

#[derive(Identifiable, Selectable, PartialEq, Serialize, Deserialize, Debug, Clone, Queryable)]
#[cfg_attr(feature = "cli", derive(tabled::Tabled))]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[serde(skip_serializing)]
    pub id: i32,
    pub name: String,
    pub password: String,
    #[serde(rename = "type")]
    pub user_role: Role,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::database::schema::user_mounts_points)]
pub struct NewUserMountPoint {
    pub user_id: i32,
    pub path: String,
}
#[cfg_attr(feature = "cli", derive(tabled::Tabled))]
#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = crate::database::schema::user_mounts_points)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserMountPoint {
    pub id: i32,
    pub user_id: i32,
    pub path: String,
}
