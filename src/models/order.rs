use chrono::{NaiveDate, NaiveDateTime};
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{Date, Integer, Text, Timestamp};
use serde::Serialize;

#[derive(Queryable, Serialize, QueryableByName)]
pub struct Order {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub website: String,
    #[diesel(sql_type = Text)]
    pub details: String,
    #[diesel(sql_type = Date)]
    pub deadline_date: NaiveDate,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
}

// Plain struct for form handling, without Diesel traits
pub struct NewOrder {
    pub website: String,
    pub details: String,
    pub deadline: String,
}
