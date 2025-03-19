use rocket_dyn_templates::{context, Template};

#[get("/login")]
pub fn login() -> Template {
    Template::render(
        "auth/login",
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
        "auth/register",
        context! {
            title: "Register",
        },
    )
}

#[get("/signup")]
pub fn signup() -> Template {
    Template::render(
        "auth/signup",
        context! {
            title: "Sign Up",
        },
    )
}

#[get("/profile")]
pub fn profile() -> Template {
    Template::render(
        "auth/profile",
        context! {
            title: "Your Profile",
        },
    )
}
