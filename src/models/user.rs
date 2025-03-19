use diesel::deserialize::QueryableByName;
use diesel::sql_types::{Integer, Text};
use serde::Serialize;

#[derive(Serialize, QueryableByName)]
pub struct User {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub email: String,
}

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
