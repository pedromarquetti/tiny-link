// @generated automatically by Diesel CLI.

diesel::table! {
    tiny_link (id) {
        id -> Int4,
        long_link -> Text,
        short_link -> Varchar,
    }
}
