// @generated automatically by Diesel CLI.

diesel::table! {
    tiny_link (id) {
        id -> Int4,
        long_link -> Text,
        short_link -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Int4>,
        user_name -> Text,
        user_role -> Nullable<Varchar>,
        user_pwd -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(tiny_link, users,);
