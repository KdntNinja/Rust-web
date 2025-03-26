use rocket::Build;
use rocket::Rocket;

use crate::controllers::{auth, dashboard, home, pricing, request};

pub fn configure_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/",
        routes![
            // Home routes
            home::index,
            // Auth routes
            auth::login,
            auth::logout,
            auth::signup,
            auth::profile,
            // Dashboard routes
            dashboard::dashboard,
            dashboard::new_order_form,
            dashboard::view_orders,
            // Service routes
            pricing::pricing,
            request::request_form,
        ],
    )
}
