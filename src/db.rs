use diesel::prelude::*;
use diesel::Connection;
use std::env;

pub fn establish_connection() -> impl Connection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    <SqliteConnection as Connection>::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
