use rocket::Request;
use rocket_dyn_templates::{context, Template};

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let path = req.uri().path().to_string();
    Template::render(
        "errors/404",
        context! {
            title: "404 - Not Found",
            path: path,
        },
    )
}

#[catch(500)]
pub fn server_error(_req: &Request) -> Template {
    Template::render(
        "errors/500",
        context! {
            title: "500 - Server Error",
            error_message: "An internal server error occurred.",
        },
    )
}
