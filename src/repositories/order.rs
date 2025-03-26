use diesel::prelude::*;
use diesel::sql_types::{Integer, Text};
use diesel::sqlite::SqliteConnection;

use crate::models::order::{NewOrder, Order};
use crate::repositories::Repository;

/// Repository for Order entity operations
///
/// This struct provides database operations for the Order entity,
/// implementing the common Repository trait.
pub struct OrderRepository;

impl Repository<Order, i32, NewOrder> for OrderRepository {
    /// Find an order by its ID
    fn find_by_id(&self, id: i32, conn: &mut SqliteConnection) -> QueryResult<Order> {
        diesel::sql_query(
            "SELECT id, website, details, deadline AS deadline_date, created_at FROM orders WHERE id = ?"
        )
        .bind::<Integer, _>(id)
        .get_result(conn)
    }

    /// Insert a new order into the database
    fn insert(&self, new_order: &NewOrder, conn: &mut SqliteConnection) -> QueryResult<Order> {
        diesel::sql_query(
            "INSERT INTO orders (website, details, deadline) 
             VALUES (?, ?, ?) 
             RETURNING id, website, details, deadline AS deadline_date, created_at",
        )
        .bind::<Text, _>(&new_order.website)
        .bind::<Text, _>(&new_order.details)
        .bind::<Text, _>(&new_order.deadline)
        .get_result(conn)
    }

    /// Find all orders
    fn find_all(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<Order>> {
        diesel::sql_query(
            "SELECT id, website, details, deadline AS deadline_date, created_at FROM orders ORDER BY created_at DESC"
        )
        .load(conn)
    }
}
