use super::ApiError;
use rocket::Request;
use rocket_dyn_templates::{context, Template};

// API error for not found
pub fn api_not_found(resource: &str) -> ApiError {
    ApiError {
        status: 404,
        message: format!("{} not found", resource),
    }
}

// Rocket-compatible catcher for 404plate {
#[catch(404)]
pub fn not_found_catcher(req: &Request) -> Template {
    Template::render(
        "errors/404",
        context! {
            title: "404 Not Found",
            uri: req.uri()
        },
    )
}

// API error for internal server error
pub fn api_internal_server_error() -> ApiError {
    ApiError {
        status: 500,
        message: "Internal server error".to_string(),
    }
}

// Rocket-compatible catcher for 500
#[catch(500)]
pub fn internal_server_error_catcher() -> Template {
    Template::render(
        "errors/500",
        context! {
            title: "500 Internal Server Error"
        },
    )
}
