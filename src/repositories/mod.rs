pub mod order;
pub mod user;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

/// A generic repository trait to standardize CRUD operations
///
/// This trait defines common methods that all repositories should implement.
/// It helps maintain consistency across different database entity operations.
pub trait Repository<T, ID, NewT> {
    /// Find an entity by its ID
    fn find_by_id(&self, id: ID, conn: &mut SqliteConnection) -> QueryResult<T>;
    
    /// Insert a new entity into the database
    fn insert(&self, new_entity: &NewT, conn: &mut SqliteConnection) -> QueryResult<T>;
    
    /// Find all entities of this type
    fn find_all(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<T>>;
}
