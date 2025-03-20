use rocket_dyn_templates::{context, Template};

#[get("/pricing")]
pub fn pricing() -> Template {
    Template::render(
        "pages/pricing",
        context! {
            title: "Our Services & Pricing",
        },
    )
}
