use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

/// Represents an order in the system
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: i32,
    pub website: String,
    pub details: String,
    pub deadline: String,
    pub created_at: NaiveDateTime,
}

/// Represents a new order to be created
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewOrder {
    pub website: String,
    pub details: String,
    pub deadline: String,
}

// Mock order database for testing
pub mod mock {
    use super::*;
    
    lazy_static! {
        static ref ORDERS: Mutex<HashMap<i32, Order>> = {
            let mut m = HashMap::new();
            // Add a sample order
            m.insert(1, Order {
                id: 1,
                website: "example.com".to_string(),
                details: "Sample order for testing".to_string(),
                deadline: "2023-12-31".to_string(),
                created_at: Local::now().naive_local(),
            });
            Mutex::new(m)
        };
    }
    
    pub fn find_all() -> Vec<Order> {
        let orders = ORDERS.lock().unwrap();
        let mut result: Vec<_> = orders.values().cloned().collect();
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        result
    }
    
    pub fn insert(new_order: NewOrder) -> Order {
        let mut orders = ORDERS.lock().unwrap();
        let id = orders.len() as i32 + 1;
        let order = Order {
            id,
            website: new_order.website,
            details: new_order.details,
            deadline: new_order.deadline,
            created_at: Local::now().naive_local(),
        };
        orders.insert(id, order.clone());
        order
    }
}
