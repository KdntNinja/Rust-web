use crate::models::user::{NewUser, User};
use diesel::prelude::*;

pub struct UserService;

impl UserService {
    pub fn find_user_by_email(email: &str, conn: &mut PgConnection) -> QueryResult<User> {
        // This would normally use a repository like OrderRepository
        diesel::sql_query("SELECT id, name, email FROM users WHERE email = $1")
            .bind::<diesel::sql_types::Text, _>(email)
            .get_result(conn)
    }

    pub fn create_user(new_user: &NewUser, conn: &mut PgConnection) -> QueryResult<User> {
        // This would normally hash the password and use a repository
        diesel::sql_query(
            "INSERT INTO users (name, email, password_hash) 
             VALUES ($1, $2, $3) 
             RETURNING id, name, email",
        )
        .bind::<diesel::sql_types::Text, _>(&new_user.name)
        .bind::<diesel::sql_types::Text, _>(&new_user.email)
        .bind::<diesel::sql_types::Text, _>(&new_user.password) // This should be hashed!
        .get_result(conn)
    }
}
