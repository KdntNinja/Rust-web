use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

/// Represents a user in the system with full details
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

/// Represents a new user to be inserted
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

// Mock user database for testing
pub mod mock {
    use super::*;
    
    lazy_static! {
        static ref USERS: Mutex<HashMap<i32, User>> = {
            let mut m = HashMap::new();
            // Add a demo user
            m.insert(1, User {
                id: 1,
                username: "demo".to_string(),
                email: "demo@example.com".to_string(),
                password_hash: "$2b$12$K2pklxM0RwiOEchkh2K.wOjlxAOQT/9f2VocbLkQwxJq/iQiv2xxu".to_string(), // "password"
                created_at: Local::now().naive_local(),
            });
            Mutex::new(m)
        };
    }
    
    pub fn find_by_email(email: &str) -> Option<User> {
        USERS.lock().unwrap().values()
            .find(|user| user.email == email)
            .cloned()
    }
    
    pub fn find_by_id(id: i32) -> Option<User> {
        USERS.lock().unwrap().get(&id).cloned()
    }
    
    pub fn insert(new_user: NewUser) -> User {
        let mut users = USERS.lock().unwrap();
        let id = users.len() as i32 + 1;
        let user = User {
            id,
            username: new_user.username,
            email: new_user.email,
            password_hash: new_user.password_hash,
            created_at: Local::now().naive_local(),
        };
        users.insert(id, user.clone());
        user
    }
}
