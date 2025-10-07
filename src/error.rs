use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

// Define an enumeration for custom application errors
#[derive(Debug)]
pub enum AppError {
    InternalServerError,      // Represents an internal server error
    BodyParsingError(String), // Represents an error related to request body parsing
}

// Define a util to create an internal server error
pub fn internal_error<E>(_err: E) -> AppError {
    AppError::InternalServerError
}

// Implement the `IntoResponse` trait for the `AppError` enumeration
impl IntoResponse for AppError {
    // Define the conversion to an Axum response
    fn into_response(self) -> Response {
        // Define status and error message based on the error variant
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
            Self::BodyParsingError(message) => (
                StatusCode::BAD_REQUEST,
                format!("Body parsing error: {}", message),
            ),
        };

        // Create a JSON response containing the error message
        (status, Json(json!({"message": err_msg}))).into_response()
    }
}
