use axum::{Json, extract::State};
use uuid::Uuid;

use crate::{
    domain::models::posts::PostError,
    handlers::posts::{PostResponse, UpdatePostRequest},
    infra::repositories::post_repository,
    state::AppState,
    utils::custom_extractors::{json_extractor::JsonExtractor, path_extractor::PathExtractor},
};

// Define the handler function for update a specific post
pub async fn update_post(
    State(state): State<AppState>, // Extract the application state from the request
    PathExtractor(post_id): PathExtractor<Uuid>,
    JsonExtractor(update_post): JsonExtractor<UpdatePostRequest>, // Extract JSON data from the request body
) -> Result<Json<PostResponse>, PostError> {
    // Create a NewPostDb instance with data from the JSON request
    let update_post_db = post_repository::UpdatePostDb {
        title: update_post.title,
        body: update_post.body,
        published: update_post.published,
    };

    // Insert the new post into the database using the repository
    let updated_post = post_repository::update(&state.pool, post_id, update_post_db)
        .await
        .map_err(PostError::InfraError)?;

    // Create a PostResponse instance from the newly created post
    let post_reponse = PostResponse {
        id: updated_post.id,
        title: updated_post.title,
        body: updated_post.body,
        published: updated_post.published,
    };

    // Return the response as JSON with a success status
    Ok(Json(post_reponse))
}
