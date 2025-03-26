pub mod guards;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text};
use diesel::sqlite::SqliteConnection;
use rocket::http::Status;
use serde::{Deserialize, Serialize};

use crate::models::user::{NewUser, User};
use crate::services::user::UserService;

/// User session data structure for authentication
///
/// This contains the minimal user information needed for authentication
/// purposes without including sensitive data like password hashes.
#[derive(Serialize, Deserialize)]
pub struct UserSession {
    pub id: i32,
    pub username: String,
    pub email: String,
}

/// Internal structure for SQL query results with password hashes
///
/// This is used internally to query the database for authentication
/// and should not be exposed outside the auth module.
#[derive(diesel::QueryableByName)]
struct UserWithHash {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    username: String,
    #[diesel(sql_type = Text)]
    email: String,
    #[diesel(sql_type = Text)]
    password_hash: String,
}

/// Hash a password using bcrypt
///
/// # Arguments
/// * `password` - The plain text password to hash
///
/// # Returns
/// The hashed password as a string
pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap_or_else(|_| String::from(""))
}

/// Verify a password against a hash
///
/// # Arguments
/// * `password` - The plain text password to check
/// * `hash` - The hashed password to compare against
///
/// # Returns
/// True if the password matches the hash, false otherwise
pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

/// Authenticate a user with email and password
///
/// # Arguments
/// * `email` - The user's email address
/// * `password` - The user's plain text password
/// * `conn` - A database connection
///
/// # Returns
/// Some(User) if authentication is successful, None otherwise
pub fn login_user(email: &str, password: &str, conn: &mut SqliteConnection) -> Option<User> {
    // Get the user and password hash from database
    let user_with_hash =
        diesel::sql_query("SELECT id, username, email, password_hash FROM users WHERE email = ?")
            .bind::<Text, _>(email)
            .get_result::<UserWithHash>(conn)
            .ok()?;

    // Verify password
    if verify_password(password, &user_with_hash.password_hash) {
        Some(User {
            id: user_with_hash.id,
            username: user_with_hash.username,
            email: user_with_hash.email,
            password_hash: user_with_hash.password_hash,
            created_at: chrono::Local::now().naive_local(),
        })
    } else {
        None
    }
}

/// Create a user session from a user model
///
/// # Arguments
/// * `user` - The user to create a session for
///
/// # Returns
/// A UserSession containing non-sensitive user data
pub fn create_user_session(user: &User) -> UserSession {
    UserSession {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
    }
}

/// Register a new user
///
/// # Arguments
/// * `new_user` - The user data to register
/// * `conn` - A database connection
///
/// # Returns
/// A Result containing the newly created user or an error status
pub fn register_user(new_user: NewUser, conn: &mut SqliteConnection) -> Result<User, Status> {
    let hashed_password = hash_password(&new_user.password_hash);

    let user_to_insert = NewUser {
        username: new_user.username,
        email: new_user.email,
        password_hash: hashed_password,
    };

    UserService::create_user(&user_to_insert, conn).map_err(|_| Status::InternalServerError)
}
