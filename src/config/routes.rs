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
                // Auth routes
                auth::login,
                auth::process_login,
                auth::logout,
                auth::signup,
                auth::process_signup,
                auth::profile,
                // Dashboard routes with auth guards
                dashboard::dashboard,
                dashboard::dashboard_redirect,
                dashboard::new_order_form,
                dashboard::new_order_form_redirect,
                dashboard::view_orders,
                dashboard::view_orders_redirect,
                dashboard::process_new_order,
                // Service routes
                pricing::pricing,
                request::request_form,
            ],
        )
        .register(
            "/",
            catchers![error_handlers::not_found, error_handlers::server_error,],
        )
}
