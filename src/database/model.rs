use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};
use tabled::Tabled;
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
            Role::Admin => {write!(f, "Admin")}
            Role::User => {write!(f, "User")}
        }
    }
}

impl FromStr for Role {

    type Err = ();

    fn from_str(input: &str) -> Result<Role, Self::Err> {
        match &*input.to_lowercase() {
            "admin"  => Ok(Role::Admin),
            "user" => Ok(Role::User),
            _      => Err(()),
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Tabled)]
#[diesel(table_name = crate::database::schema::users)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: i64,
    pub name: String,
    pub password: String,
    #[serde(rename = "type")]
    pub user_role: Role,
}
