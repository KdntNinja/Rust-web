use diesel::prelude::*;
use diesel::sql_types::Text;

use crate::models::user::{NewUser, User};

pub struct UserRepository;

impl UserRepository {
    pub fn find_by_email(email: &str, conn: &mut PgConnection) -> QueryResult<User> {
        diesel::sql_query("SELECT id, name, email FROM users WHERE email = $1")
            .bind::<Text, _>(email)
            .get_result(conn)
    }

    pub fn insert(new_user: &NewUser, conn: &mut PgConnection) -> QueryResult<User> {
        diesel::sql_query(
            "INSERT INTO users (name, email, password_hash) 
             VALUES ($1, $2, $3) 
             RETURNING id, name, email",
        )
        .bind::<Text, _>(&new_user.name)
        .bind::<Text, _>(&new_user.email)
        .bind::<Text, _>(&new_user.password) // This should be hashed!
        .get_result(conn)
    }
}
