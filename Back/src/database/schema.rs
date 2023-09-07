// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    files (path) {
        path -> Text,
        tags -> Nullable<Array<Nullable<Int8>>>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Int8,
        name -> Text,
        password -> Text,
        user_role -> Role,
    }
}

diesel::allow_tables_to_appear_in_same_query!(files, tags, users,);
