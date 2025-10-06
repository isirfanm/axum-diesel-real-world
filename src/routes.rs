use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

use crate:state::AppState;

// Function to create the main application router
pub fn app_router(state: AppState) -> Router {
    Router::new()
        // Define the root route
        .route("/", get(root))
        // Define a fallback handler for 404 errors
        .fallback(handler_404)
        // Attach the application state to the router
        .with_state(state)
}

// Handler for the root route
async fn root() -> &'static str {
    "Server is running!"
}

// Handler for 404 Not Found errors
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found.",
    )
}
