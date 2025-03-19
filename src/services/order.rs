use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::sql_types::Text;

use crate::models::order::{NewOrder, Order};
use crate::repositories::order::OrderRepository;

pub struct OrderService;

impl OrderService {
    pub fn create_order(new_order: &NewOrder, conn: &mut PgConnection) -> QueryResult<Order> {
        OrderRepository::insert(new_order, conn)
    }

    pub fn get_all_orders(conn: &mut PgConnection) -> QueryResult<Vec<Order>> {
        OrderRepository::find_all(conn)
    }

    pub fn get_order_by_id(id: i32, conn: &mut PgConnection) -> QueryResult<Order> {
        OrderRepository::find_by_id(id, conn)
    }
}
