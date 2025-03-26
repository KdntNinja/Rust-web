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

#[get("/signup")]
pub fn signup() -> Template {
    Template::render(
        "pages/auth/signup",
        context! {
            title: "Sign Up",
        },
    )
}

#[get("/profile")]
pub fn profile() -> Template {
    Template::render(
        "pages/auth/profile",
        context! {
            title: "Your Profile",
        },
    )
}
