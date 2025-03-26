use crate::config::routes::auth::{login, logout, profile, register};
use crate::config::routes::dashboard::{dashboard, new_order, view_orders};
use crate::config::routes::pages::{index, not_found, pricing, server_error};
use rocket::{Build, Rocket};

pub fn configure_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        // Main pages
        .mount("/", routes![index, pricing])
        // Auth routes
        .mount("/", routes![login, logout, register, profile])
        // Dashboard routes
        .mount("/", routes![dashboard, new_order, view_orders])
        // Error catchers
        .register("/", catchers![not_found, server_error])
}
