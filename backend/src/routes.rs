use rocket::get;

#[get("/")]
pub fn index() -> &'static str {
    "Hello from routes module!"
}

#[get("/hello/<name>")]
pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
