pub mod guards;

use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::http::Status;
use serde::{Deserialize, Serialize};

use crate::models::user::{NewUser, User, mock};

#[derive(Serialize, Deserialize)]
pub struct UserSession {
    pub id: i32,
    pub username: String,
    pub email: String,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap_or_else(|_| String::from(""))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

/// Authenticate a user with email and password using mock data
pub fn login_user(email: &str, password: &str) -> Option<User> {
    let user = mock::find_by_email(email)?;
    
    if verify_password(password, &user.password_hash) {
        Some(user)
    } else {
        None
    }
}


/// Register a new user using mock data
pub fn register_user(mut new_user: NewUser) -> Result<User, Status> {
    // Check if email already exists
    if mock::find_by_email(&new_user.email).is_some() {
        return Err(Status::Conflict);
    }
    
    let hashed_password = hash_password(&new_user.password_hash);
    new_user.password_hash = hashed_password;
    
    Ok(mock::insert(new_user))
}
