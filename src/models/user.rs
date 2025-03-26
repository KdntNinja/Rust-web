use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

/// Represents a user in the system with full details
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

/// Represents a new user to be inserted into the database
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    /// Find a user by their email address
    pub fn find_by_email(email_query: &str, conn: &mut SqliteConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users.filter(email.eq(email_query)).first(conn)
    }

    /// Find a user by their ID
    pub fn find_by_id(user_id: i32, conn: &mut SqliteConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users.find(user_id).first(conn)
    }
}
