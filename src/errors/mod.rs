pub mod database;
pub mod validation;
pub mod general;

use rocket::http::Status;
use rocket::response::{self, Responder};
use rocket::Request;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub status: u16,
    pub message: String,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        rocket::response::Response::build()
            .status(Status::new(self.status))
            .sized_body(self.message.len(), std::io::Cursor::new(self.message))
            .ok()
    }
}
