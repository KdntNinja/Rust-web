use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::order::{NewOrder, Order};
use crate::repositories::order::OrderRepository;
use crate::repositories::Repository;
use crate::services::Service;

/// Service for Order-related business logic
///
/// This service handles all business logic related to orders,
/// mediating between controllers and the order repository.
pub struct OrderService {
    repository: OrderRepository,
}

impl OrderService {
    /// Create a new OrderService instance
    pub fn new() -> Self {
        Self {
            repository: OrderRepository,
        }
    }
}

impl Service<Order, i32, NewOrder> for OrderService {
    fn get_by_id(&self, id: i32, conn: &mut SqliteConnection) -> QueryResult<Order> {
        self.repository.find_by_id(id, conn)
    }
    
    fn create(&self, new_order: &NewOrder, conn: &mut SqliteConnection) -> QueryResult<Order> {
        self.repository.insert(new_order, conn)
    }
    
    fn get_all(&self, conn: &mut SqliteConnection) -> QueryResult<Vec<Order>> {
        self.repository.find_all(conn)
    }
}

// Create a single static instance for convenience
impl OrderService {
    /// Get a reference to the order service
    ///
    /// This provides a convenient way to get an OrderService instance.
    pub fn instance() -> Self {
        Self::new()
    }
    
    /// Convenience method to create an order
    ///
    /// This static method allows calling OrderService::create_order() without
    /// explicitly creating an OrderService instance.
    pub fn create_order(new_order: &NewOrder, conn: &mut SqliteConnection) -> QueryResult<Order> {
        Self::instance().create(new_order, conn)
    }
    
    /// Convenience method to get all orders
    pub fn get_all_orders(conn: &mut SqliteConnection) -> QueryResult<Vec<Order>> {
        Self::instance().get_all(conn)
    }
    
    /// Convenience method to get an order by ID
    pub fn get_order_by_id(id: i32, conn: &mut SqliteConnection) -> QueryResult<Order> {
        Self::instance().get_by_id(id, conn)
    }
}
