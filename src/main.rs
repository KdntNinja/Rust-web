#[macro_use]
extern crate rocket;

mod auth;
mod config;
mod controllers;
mod error_handlers;
mod models;
// Remove Repository & Service implementations that use diesel
// mod repositories;
// mod services;

use config::routes::configure_routes;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // No database setup needed anymore

    let rocket = rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")));

    configure_routes(rocket)
}
