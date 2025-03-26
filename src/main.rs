#[macro_use]
extern crate rocket;

mod config;
mod controllers;
mod db;
mod docker;
mod errors;
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
    if let Err(e) = docker::ensure_postgres_running() {
        eprintln!("Error: Could not ensure PostgreSQL is running: {}", e);
    }

    // Run migrations
    if let Err(e) = docker::run_migrations() {
        eprintln!("Error: Failed to run migrations: {}", e);
    }

    let rocket = rocket::build()
        .attach(DbConn::fairing())
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/pricing", routes![controllers::pricing::pricing])
        .register(
            "/",
            catchers![
                errors::general::not_found_catcher,
                errors::general::internal_server_error_catcher
            ],
        )
        .attach(Template::fairing());

    configure_routes(rocket)
}
