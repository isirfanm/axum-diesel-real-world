use axum::{Json, extract::State};

use crate::{
    domain::models::posts::PostError,
    handlers::posts::{CreatePostRequest, PostResponse},
    infra::repositories::post_repository,
    state::AppState,
};

// Define the handler function for creating a new post
pub async fn create_post(
    State(state): State<AppState>, // Extract the application state from the request
    JsonExtractor(new_post): JsonExtractor<CreatePostRequest>, // Extract JSON data from the request body
) -> Result<Json<PostResponse>, PostError> {
    // Create a NewPostDb instance with data from the JSON request
    let new_post_db = post_repository::NewPostDb {
        title: new_post.title,
        body: new_post.body,
        published: false, // Set initial 'published' status to false
    };

    // Insert the new post into the database using the repository
    let created_post = post_repository::insert(&state.pool, new_post_db)
        .await
        .map_err(PostError::InfraError)?;

    // Create a PostResponse instance from the newly created post
    let post_reponse = PostResponse {
        id: created_post.id,
        title: created_post.title,
        body: created_post.body,
        published: created_post.published,
    };

    // Return the response as JSON with a success status
    Ok(Json(post_reponse))
}
