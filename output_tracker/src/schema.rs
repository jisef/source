// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Date,
        amount -> Float8,
        reason -> Text,
    }
}
