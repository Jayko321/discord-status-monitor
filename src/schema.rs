// @generated automatically by Diesel CLI.

diesel::table! {
    logs (id) {
        id -> Integer,
        status -> Text,
        activity -> Text,
        user_id -> BigInt,
        unix_time -> BigInt,
    }
}
