use rocket_dyn_templates::{context, Template};

#[get("/login")]
pub fn login() -> Template {
    Template::render(
        "pages/auth/login",
        context! {
            title: "Login",
        },
    )
}

#[get("/logout")]
pub fn logout() -> &'static str {
    "You have been logged out."
}

#[get("/register")]
pub fn register() -> Template {
    Template::render(
        "pages/auth/register",
        context! {
            title: "Register",
        },
    )
}

#[get("/profile")]
pub fn profile() -> Template {
    Template::render(
        "pages/profile",
        context! {
            title: "Your Profile",
        },
    )
}
