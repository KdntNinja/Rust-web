use diesel::table;

table! {
    orders (id) {
        id -> Int4,
        website -> Varchar,
        details -> Text,
        deadline -> Date,
        created_at -> Timestamp,
    }
}
