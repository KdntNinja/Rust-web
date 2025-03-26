pub mod order;
pub mod user;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

/// Generic service trait for business logic operations
///
/// This trait standardizes common service operations that work with repositories.
/// It provides a layer between controllers and repositories for business logic.
pub trait Service<T, ID, NewT> {
    /// Get an entity by its ID
    fn get_by_id(&self, id: ID, conn: &mut SqliteConnection) -> QueryResult<T>;

    /// Create a new entity
    fn create(&self, new_entity: &NewT, conn: &mut SqliteConnection) -> QueryResult<T>;

    /// Get all entities
    fn get_all(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<T>>;
}
