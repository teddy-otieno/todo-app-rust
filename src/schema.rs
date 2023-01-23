// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Int4,
        item -> Text,
        done -> Bool,
    }
}
