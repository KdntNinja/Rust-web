use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Home - Automatic Homework Completion",
        },
    )
}

#[get("/about")]
pub fn about() -> Template {
    Template::render(
        "pages/about",
        context! {
            title: "About Us - Learn More About Our Service",
        },
    )
}
