// @generated automatically by Diesel CLI.

diesel::table! {
    ausgabe (id) {
        id -> Int4,
        amount -> Float8,
        category -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Date,
        amount -> Float8,
        reason -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ausgabe,
    users,
);
