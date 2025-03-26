use rocket_dyn_templates::{context, Template};

#[get("/request")]
pub fn request_form() -> Template {
    Template::render(
        "pages/request_form",
        context! {
            title: "Request Homework Help",
        },
    )
}
