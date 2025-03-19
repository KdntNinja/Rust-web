use super::ApiError;

pub fn connection_error() -> ApiError {
    ApiError {
        status: 500,
        message: "Database connection error".to_string(),
    }
}

pub fn query_error(details: &str) -> ApiError {
    ApiError {
        status: 500,
        message: format!("Database query error: {}", details),
    }
}
