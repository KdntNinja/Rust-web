pub mod guards;

use crate::models::user::{NewUser, User};
use crate::services::user::UserService;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::result::Error;
use diesel::sql_types::{Integer, Text};
use diesel::PgConnection;
use diesel::RunQueryDsl;

// Define a struct for SQL query results with QueryableByName
#[derive(diesel::QueryableByName)]
struct UserWithHash {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    name: String,
    #[diesel(sql_type = Text)]
    email: String,
    #[diesel(sql_type = Text)]
    password_hash: String,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap_or_else(|_| String::from(""))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

pub fn login_user(email: &str, password: &str, conn: &mut PgConnection) -> Result<User, Error> {
    // Get the user and password hash from database
    let user_with_hash =
        diesel::sql_query("SELECT id, name, email, password_hash FROM users WHERE email = $1")
            .bind::<diesel::sql_types::Text, _>(email)
            .get_result::<UserWithHash>(conn)?;

    // Verify password
    if verify_password(password, &user_with_hash.password_hash) {
        Ok(User {
            id: user_with_hash.id,
            name: user_with_hash.name,
            email: user_with_hash.email,
        })
    } else {
        Err(Error::NotFound)
    }
}

pub fn register_user(new_user: NewUser, conn: &mut PgConnection) -> Result<User, Error> {
    let hashed_password = hash_password(&new_user.password);

    let user_with_hashed_pw = NewUser {
        name: new_user.name,
        email: new_user.email,
        password: hashed_password,
    };

    UserService::create_user(&user_with_hashed_pw, conn)
}
