// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "settings_method"))]
    pub struct SettingsMethod;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "token_purpose"))]
    pub struct TokenPurpose;
}

diesel::table! {
    credentials (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 512]
        token -> Varchar,
        is_recovery -> Bool,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SettingsMethod;

    settings_audit (id) {
        id -> Uuid,
        user_id -> Uuid,
        method -> SettingsMethod,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        geo_location -> Nullable<Varchar>,
        active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TokenPurpose;

    verification_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 512]
        token -> Varchar,
        #[max_length = 16]
        code -> Nullable<Varchar>,
        purpose -> TokenPurpose,
        expires_at -> Timestamptz,
        used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(credentials -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(settings_audit -> users (user_id));
diesel::joinable!(verification_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    credentials,
    sessions,
    settings_audit,
    users,
    verification_tokens,
);
