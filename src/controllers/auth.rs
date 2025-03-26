use rocket::form::Form;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::auth::{login_user, register_user};
use crate::models::user::NewUser;
use crate::DbConn;

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
        "pages/auth/login",
        context! {
            title: "Login",
        },
    )
}

#[post("/login", data = "<form>")]
pub async fn process_login(
    form: Form<LoginForm>,
    cookies: &CookieJar<'_>,
    conn: DbConn,
) -> Result<Redirect, Template> {
    let login_data = form.into_inner();

    let result = conn
        .run(move |c| login_user(&login_data.email, &login_data.password, c))
        .await;

    match result {
        Ok(user) => {
            // Set session cookie
            let cookie = Cookie::build("user_id", user.id.to_string())
                .path("/")
                .build();
            cookies.add_private(cookie);
            Ok(Redirect::to("/dashboard"))
        }
        Err(_) => {
            // Login failed
            Err(Template::render(
                "pages/auth/login",
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
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/")
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

#[post("/signup", data = "<form>")]
pub async fn process_signup(
    form: Form<SignupForm>,
    cookies: &CookieJar<'_>,
    conn: DbConn,
) -> Result<Redirect, Template> {
    let signup_data = form.into_inner();

    let new_user = NewUser {
        name: signup_data.name,
        email: signup_data.email,
        password: signup_data.password,
    };

    let result = conn.run(move |c| register_user(new_user, c)).await;

    match result {
        Ok(user) => {
            // Set session cookie
            let cookie = Cookie::build("user_id", user.id.to_string())
                .path("/")
                .build();
            cookies.add_private(cookie);
            Ok(Redirect::to("/dashboard"))
        }
        Err(_) => {
            // Registration failed
            Err(Template::render(
                "pages/auth/signup",
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
        "pages/auth/profile",
        context! {
            title: "Your Profile",
        },
    )
}
