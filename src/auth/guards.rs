use crate::models::user::User as DbUser;
use crate::DbConn;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug)]
pub enum AuthError {
    DatabaseError,
    UserNotFound,
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

        // Get database connection
        let conn = request.guard::<DbConn>().await.succeeded();
        if let Some(conn) = conn {
            // Query the database to get user details
            match conn.run(move |c| DbUser::find_by_id(user_id, c)).await {
                Ok(db_user) => Outcome::Success(User {
                    id: db_user.id,
                    username: db_user.username,
                    email: db_user.email,
                }),
                Err(_) => Outcome::Forward(Status::Unauthorized),
            }
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}
