use axum::{Json, extract::State};
use uuid::Uuid;

use crate::{
    domain::models::posts::{PostError, PostModel},
    handlers::posts::PostResponse,
    infra::{errors::InfraError, repositories::post_repository},
    state::AppState,
    utils::custom_extractors::path_extractor::PathExtractor,
};

// Define the handler function for retrieving a specific post by ID
pub async fn get_post(
    State(state): State<AppState>, // Extract the application state from the request
    PathExtractor(post_id): PathExtractor<Uuid>, // Extract the post_id from the request path
) -> Result<Json<PostResponse>, PostError> {
    // Use the post_repository to fetch the post based on its ID
    let post =
        post_repository::get(&state.pool, post_id)
            .await
            .map_err(|db_error| match db_error {
                InfraError::InternalServerError => PostError::InternalServerError,
                InfraError::NotFound => PostError::NotFound(post_id),
            })?;

    // Convert the retrieved PostModel to a PostReponse
    Ok(Json(adapt_post_to_post_response(post)))
}

// Helper function to adapt a PostModel to a PostResponse
fn adapt_post_to_post_response(post: PostModel) -> PostResponse {
    PostResponse {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    }
}
