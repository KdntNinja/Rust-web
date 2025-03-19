use super::ApiError;

pub fn missing_field(field: &str) -> ApiError {
    ApiError {
        status: 400,
        message: format!("Missing required field: {}", field),
    }
}

pub fn invalid_format(field: &str) -> ApiError {
    ApiError {
        status: 400,
        message: format!("Invalid format for field: {}", field),
    }
}
