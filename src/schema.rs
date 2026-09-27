// @generated automatically by Diesel CLI.

diesel::table! {
    presences (user_id) {
        user_id -> BigInt,
        status -> Text,
        activities -> Text,
        unix_time -> BigInt,
    }
}
