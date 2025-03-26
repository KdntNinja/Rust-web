// Import diesel SQL types specifically
use diesel::sql_types::{Integer, Text, Date, Timestamp};
use diesel::allow_tables_to_appear_in_same_query;
use diesel::table;

table! {
    orders (id) {
        id -> Integer,
        website -> Text,
        details -> Text,
        deadline -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(orders, users);
