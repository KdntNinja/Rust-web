use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub enum AuthError {
    NotLoggedIn,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get user from cookies/session
        let user = request.cookies().get_private("user_id").and_then(|cookie| {
            // Parse the cookie value as i32
            cookie.value().parse::<i32>().ok().map(|id| {
                // For simplicity, we'll just create a user with the ID from the cookie
                // In a real app, you would query the database to get user details
                User {
                    id,
                    name: "".to_string(),
                    email: "".to_string(),
                }
            })
        });

        match user {
            Some(user) => Outcome::Success(user),
            None => Outcome::Error((Status::Unauthorized, AuthError::NotLoggedIn)),
        }
    }
}
