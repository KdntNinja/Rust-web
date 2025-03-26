use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::user::{NewUser, User};
use crate::repositories::user::UserRepository;
use crate::repositories::Repository;
use crate::services::Service;

/// Service for User-related business logic
///
/// This service encapsulates all business logic related to users,
/// acting as an intermediary between controllers and repositories.
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    /// Create a new UserService instance
    pub fn new() -> Self {
        Self {
            repository: UserRepository,
        }
    }

    /// Find a user by their email
    ///
    /// # Arguments
    /// * `email` - The email to search for
    /// * `conn` - A database connection
    pub fn find_user_by_email(
        &self,
        email: &str,
        conn: &mut SqliteConnection,
    ) -> QueryResult<User> {
        self.repository.find_by_email(email, conn)
    }
}

impl Service<User, i32, NewUser> for UserService {
    fn get_by_id(&self, id: i32, conn: &mut SqliteConnection) -> QueryResult<User> {
        self.repository.find_by_id(id, conn)
    }

    fn create(&self, new_user: &NewUser, conn: &mut SqliteConnection) -> QueryResult<User> {
        self.repository.insert(new_user, conn)
    }

    fn get_all(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<User>> {
        self.repository.find_all(conn)
    }
}

// Create a single static instance for convenience
impl UserService {
    /// Get a reference to the user service
    ///
    /// This is a convenience function to get a reference to a UserService
    /// instance without having to create one.
    pub fn instance() -> Self {
        Self::new()
    }

    /// A convenience function to create a user
    ///
    /// This static method lets you call UserService::create_user() without
    /// explicitly creating a UserService instance.
    pub fn create_user(new_user: &NewUser, conn: &mut SqliteConnection) -> QueryResult<User> {
        Self::instance().create(new_user, conn)
    }
}
