mod routes;
mod api;

use rocket::{get, launch, routes, fs::FileServer};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Page {
    title: String,
}

// API endpoint example
#[get("/api/status")]
fn api_status() -> rocket::serde::json::Json<serde_json::Value> {
    rocket::serde::json::Json(serde_json::json!({
        "status": "online",
        "version": "0.1.0"
    }))
}

#[get("/")]
fn index() -> String {
    "Welcome to my Rust Web Application!".to_string()
}

#[get("/hello")]
fn hello() -> String {
    "Hello, Rocket!".to_string()
}

#[get("/<path>")]
fn path_handler(path: String) -> String {
    format!("Hit path: {}", path)
}

#[launch]
fn rocket() -> _ {
    // Configure CORS
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put, Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        // Mount API routes
        .mount("/", routes![api_status])
        .mount("/api", routes![
            api::users::get_users,
            api::users::get_user
        ])
        // Mount frontend static files
        .mount("/", FileServer::from("../frontend/dist"))
        .mount("/", routes![index, hello, path_handler])
}
