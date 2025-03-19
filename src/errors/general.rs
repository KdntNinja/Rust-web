use super::ApiError;

pub fn not_found(resource: &str) -> ApiError {
    ApiError {
        status: 404,
        message: format!("{} not found", resource),
    }
}

pub fn internal_server_error() -> ApiError {
    ApiError {
        status: 500,
        message: "Internal server error".to_string(),
    }
}
