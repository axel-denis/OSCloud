// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    user_mounts_points (id) {
        id -> Int4,
        user_id -> Int4,
        path -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Int4,
        name -> Text,
        password -> Text,
        user_role -> Role,
    }
}

diesel::joinable!(user_mounts_points -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(user_mounts_points, users,);
