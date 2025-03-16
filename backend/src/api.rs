pub mod users {
    use rocket::serde::json::Json;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        id: u64,
        name: String,
        email: String,
    }

    #[rocket::get("/users")]
    pub fn get_users() -> Json<Vec<User>> {
        // This would normally fetch from a database
        let users = vec![
            User {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Jane Smith".to_string(),
                email: "jane@example.com".to_string(),
            },
        ];
        
        Json(users)
    }

    #[rocket::get("/users/<id>")]
    pub fn get_user(id: u64) -> Option<Json<User>> {
        // This would normally fetch from a database
        if id == 1 {
            Some(Json(User {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
            }))
        } else if id == 2 {
            Some(Json(User {
                id: 2, 
                name: "Jane Smith".to_string(),
                email: "jane@example.com".to_string(),
            }))
        } else {
            None
        }
    }
}
