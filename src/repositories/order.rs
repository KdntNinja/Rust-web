use diesel::prelude::*;
use diesel::sql_types::Text;

use crate::models::order::{NewOrder, Order};

pub struct OrderRepository;

impl OrderRepository {
    pub fn insert(new_order: &NewOrder, conn: &mut PgConnection) -> QueryResult<Order> {
        // Use prepared statement-style query to prevent SQL injection
        diesel::sql_query(
            "INSERT INTO orders (website, details, deadline) 
             VALUES ($1, $2, $3) 
             RETURNING id, website, details, deadline AS deadline_date, created_at",
        )
        .bind::<Text, _>(&new_order.website)
        .bind::<Text, _>(&new_order.details)
        .bind::<Text, _>(&new_order.deadline) // Use string representation of date
        .get_result(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Order>> {
        diesel::sql_query(
            "SELECT id, website, details, deadline AS deadline_date, created_at FROM orders ORDER BY created_at DESC"
        )
        .load(conn)
    }

    pub fn find_by_id(id: i32, conn: &mut PgConnection) -> QueryResult<Order> {
        diesel::sql_query(
            "SELECT id, website, details, deadline AS deadline_date, created_at FROM orders WHERE id = $1"
        )
        .bind::<diesel::sql_types::Integer, _>(id)
        .get_result(conn)
    }
}
