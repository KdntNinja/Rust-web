use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::auth::{login_user, register_user};
use crate::models::user::NewUser;

#[derive(FromForm)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[derive(FromForm)]
pub struct SignupForm {
    name: String,
    email: String,
    password: String,
}

#[get("/login")]
pub fn login() -> Template {
    Template::render(
        "auth/login",
        context! {
            title: "Login",
        },
    )
}

#[post("/login", data = "<form>")]
pub fn process_login(
    form: Form<LoginForm>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Template> {
    let login_data = form.into_inner();

    match login_user(&login_data.email, &login_data.password) {
        Some(user) => {
            // Set session cookie with the user ID
            let cookie = Cookie::new("user_id", user.id.to_string());
            cookies.add_private(cookie);
            Ok(Redirect::to("/dashboard"))
        }
        None => {
            // Login failed
            Err(Template::render(
                "auth/login",
                context! {
                    title: "Login",
                    error: "Invalid email or password. Please try again.",
                },
            ))
        }
    }
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::new("user_id", ""));
    Redirect::to("/")
}

#[get("/signup")]
pub fn signup_page() -> Template {
    Template::render(
        "auth/register",
        context! {
            title: "Sign Up",
        },
    )
}

#[post("/signup", data = "<signup_data>")]
pub fn process_signup(
    signup_data: Form<SignupForm>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Template> {
    let signup_data = signup_data.into_inner();

    let new_user = NewUser {
        username: signup_data.name,
        email: signup_data.email,
        password_hash: signup_data.password,
    };

    match register_user(new_user) {
        Ok(user) => {
            // Set session cookie with the user ID
            let cookie = Cookie::new("user_id", user.id.to_string());
            cookies.add_private(cookie);
            Ok(Redirect::to("/dashboard"))
        }
        Err(_) => {
            // Registration failed
            Err(Template::render(
                "auth/register",
                context! {
                    title: "Sign Up",
                    error: "Registration failed. Email may already be in use.",
                },
            ))
        }
    }
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
