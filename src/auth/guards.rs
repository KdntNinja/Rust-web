use crate::models::user::mock;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

// Simplify to a single error type since we're not using the variants
#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get user ID from cookies/session
        let user_id = match request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse::<i32>().ok())
        {
            Some(id) => id,
            None => return Outcome::Forward(Status::Unauthorized),
        };

        // Get user from mock database
        match mock::find_by_id(user_id) {
            Some(db_user) => Outcome::Success(User {
                id: db_user.id,
                username: db_user.username,
                email: db_user.email,
            }),
            None => Outcome::Forward(Status::Unauthorized),
        }
    }
}
