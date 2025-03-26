use rocket_dyn_templates::{context, Template};

#[get("/pricing")]
pub fn pricing() -> Template {
    Template::render(
        "pricing",
        context! {
            title: "Our Services & Pricing",
        },
    )
}
