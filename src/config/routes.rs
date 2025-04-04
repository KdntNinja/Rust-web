use rocket::Build;
use rocket::Rocket;

use crate::controllers::{auth, dashboard, home, pricing, request};
use crate::error_handlers;

pub fn configure_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount(
            "/",
            routes![
                // Home routes
                home::index,
                home::about,
                
                // Auth routes
                auth::login,
                auth::process_login,
                auth::logout,
                auth::signup_page,
                auth::process_signup,
                auth::profile,
                
                // Dashboard routes
                dashboard::dashboard,
                dashboard::dashboard_redirect,
                dashboard::view_orders,
                dashboard::view_orders_redirect,
                dashboard::new_order_form,
                dashboard::new_order_form_redirect,
                dashboard::process_new_order,
                
                // Pricing routes
                pricing::pricing,
                
                // Request routes
                request::request_form,
            ],
        )
        .register(
            "/",
            catchers![error_handlers::not_found, error_handlers::server_error],
        )
}
