// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "share_type"))]
    pub struct ShareType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ShareType;

    files_shares (id) {
        id -> Int4,
        owner_user_id -> Int4,
        path -> Text,
        share_type -> ShareType,
        link -> Text,
    }
}

diesel::table! {
    files_shares_users (id) {
        id -> Int4,
        file_share_id -> Int4,
        shared_to -> Int4,
    }
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
        enabled -> Bool,
    }
}

diesel::joinable!(files_shares -> users (owner_user_id));
diesel::joinable!(files_shares_users -> files_shares (file_share_id));
diesel::joinable!(files_shares_users -> users (shared_to));
diesel::joinable!(user_mounts_points -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    files_shares,
    files_shares_users,
    user_mounts_points,
    users,
);
