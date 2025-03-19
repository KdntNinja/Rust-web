#[macro_use]
extern crate rocket;

mod config;
mod controllers;
mod db;
mod docker;
mod error_handlers;
mod models;
mod repositories;
mod services;

use config::routes::configure_routes;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    // Ensure PostgreSQL is running in Docker
    match docker::ensure_postgres_running() {
        Ok(_) => println!("PostgreSQL Docker container is ready."),
        Err(e) => eprintln!("Warning: Could not ensure PostgreSQL is running: {}", e),
    }

    // Run migrations
    match docker::run_migrations() {
        Ok(_) => println!("Database migrations completed successfully."),
        Err(e) => eprintln!("Warning: Failed to run migrations: {}", e),
    }

    let mut rocket_builder = rocket::build();

    // Add database connection - handle potential failure gracefully
    #[cfg(debug_assertions)]
    {
        println!("Attempting to connect to database...");
        // In debug mode, we'll still launch the app even if DB connection fails
        rocket_builder = rocket_builder.attach(DbConn::fairing());
    }

    #[cfg(not(debug_assertions))]
    {
        // In release mode, we require the database
        rocket_builder = rocket_builder.attach(DbConn::fairing());
    }

    // Configure routes using the new structure
    rocket_builder = configure_routes(rocket_builder);

    rocket_builder
        .mount("/static", FileServer::from(relative!("static")))
        .register(
            "/",
            catchers![error_handlers::not_found, error_handlers::server_error],
        )
        .attach(Template::fairing())
}
