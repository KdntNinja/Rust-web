extern crate diesel;
extern crate rocket;

use diesel::sqlite::SqliteConnection;
use rocket_sync_db_pools::database;

// Create a SQLite database connection pool
#[database("sqlite_db")]
pub struct DbConn(SqliteConnection);

pub mod db;
pub mod models;
pub mod schema;
