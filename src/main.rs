#[macro_use]
extern crate rocket;

mod auth;
mod config;
mod controllers;
mod db;
mod error_handlers;
mod models;
mod repositories;
mod services;

use config::routes::configure_routes;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[database("sqlite_db")]
pub struct DbConn(SqliteConnection);

/// Run SQLite migrations to set up the database schema
fn run_sqlite_migrations() -> Result<(), Box<dyn Error>> {
    use diesel::connection::SimpleConnection;
    
    let mut conn = diesel::sqlite::SqliteConnection::establish("database.sqlite")?;
    conn.batch_execute("PRAGMA foreign_keys = ON")?;
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Run SQLite migrations
    if let Err(e) = run_sqlite_migrations() {
        eprintln!("\n❌ SQLite Migration Error: {}\n", e);
        eprintln!("The application will continue to run, but database functionality may be limited.");
    } else {
        println!("✅ SQLite Database migrations completed successfully!");
    }

    let rocket = rocket::build()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")));

    configure_routes(rocket)
}
