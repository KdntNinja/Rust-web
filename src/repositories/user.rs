use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::sqlite::SqliteConnection;

use crate::models::user::{NewUser, User};
use crate::repositories::Repository;

/// Repository for User entity operations
///
/// This struct provides database operations for the User entity,
/// implementing the common Repository trait and additional user-specific methods.
pub struct UserRepository;

impl Repository<User, i32, NewUser> for UserRepository {
    /// Find a user by their ID
    fn find_by_id(&self, id: i32, conn: &mut SqliteConnection) -> QueryResult<User> {
        diesel::sql_query(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE id = ?",
        )
        .bind::<diesel::sql_types::Integer, _>(id)
        .get_result(conn)
    }

    /// Insert a new user into the database
    fn insert(&self, new_user: &NewUser, conn: &mut SqliteConnection) -> QueryResult<User> {
        diesel::sql_query("INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?)")
            .bind::<Text, _>(&new_user.username)
            .bind::<Text, _>(&new_user.email)
            .bind::<Text, _>(&new_user.password_hash)
            .execute(conn)?;

        diesel::sql_query(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE email = ?",
        )
        .bind::<Text, _>(&new_user.email)
        .get_result(conn)
    }

    /// Find all users
    fn find_all(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<User>> {
        diesel::sql_query("SELECT id, username, email, password_hash, created_at FROM users")
            .load(conn)
    }
}

impl UserRepository {
    /// Find a user by their email
    ///
    /// This is a user-specific operation that extends the base Repository trait.
    pub fn find_by_email(&self, email: &str, conn: &mut SqliteConnection) -> QueryResult<User> {
        diesel::sql_query(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE email = ?",
        )
        .bind::<Text, _>(email)
        .get_result(conn)
    }
}
