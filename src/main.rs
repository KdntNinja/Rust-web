#[macro_use]
extern crate rocket;

mod error_handlers;
mod models;
mod routes;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::index::index, routes::hello::hello])
        .mount("/static", FileServer::from(relative!("static")))
        .register(
            "/",
            catchers![error_handlers::not_found, error_handlers::server_error],
        )
        .attach(Template::fairing())
}
