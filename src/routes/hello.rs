use rocket_dyn_templates::{context, Template};

#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render(
        "hello",
        context! {
            title: "Hello",
            name: name,
        },
    )
}
