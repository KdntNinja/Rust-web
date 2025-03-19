#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
    name: String,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Rocket Web App",
            name: "Rocket User",
        },
    )
}

#[get("/hello/<name>")]
fn hello(name: &str) -> Template {
    Template::render(
        "hello",
        context! {
            title: "Hello",
            name: name,
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
