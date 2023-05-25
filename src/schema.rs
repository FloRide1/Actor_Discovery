// @generated automatically by Diesel CLI.

diesel::table! {
    persistencelog (id) {
        id -> Int4,
        content -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(persistencelog, users,);
