use axum::{
    extract::{Json, Path, State}, response::IntoResponse
};
use validator::Validate;
use crate::{db::DbPool, models::{api_response::ApiResponse, post::NewPost}, repositories::{post::PostRepositoryTrait, post_repository::PostRepository}};

pub async fn list_posts(
    State(pool): State<DbPool>,
) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let mut repo = PostRepository::new(&mut conn);

    match repo.list_all() {
        Ok(posts) => {
            let response = ApiResponse {
                status: 200,
                message: "OK".to_string(),
                data: Some(posts),
            };
            Json(response).into_response()
        },
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch posts").into_response(),
    }
}

pub async fn get_post(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let mut repo = PostRepository::new(&mut conn);

    match repo.find_by_id(id) {
        Ok(post) => {
            let response = ApiResponse {
                status: 200,
                message: "OK".to_string(),
                data: Some(post),
            };
            Json(response).into_response()
        },
        Err(_) => (axum::http::StatusCode::NOT_FOUND, "Post not found").into_response(),
    }
}

pub async fn create_post(
    State(pool): State<DbPool>,
    Json(payload): Json<NewPost>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return (axum::http::StatusCode::BAD_REQUEST, {
            let errs = 
                e.field_errors()
                    .iter()
                    .map(|(field, errors)| {
                        (field.to_string(), errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>())
                    })
                    .collect::<serde_json::Value>();
            let response = ApiResponse {
                status: 400,
                message: "Validation Error".to_string(),
                data: Some(errs),
            };
            Json(response)
        }).into_response();
    }
    let mut conn = pool.get().expect("Failed to get DB connection");
    let mut repo = PostRepository::new(&mut conn);

    match repo.create(payload) {
        Ok(post) => (axum::http::StatusCode::CREATED, Json(post)).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to create post").into_response(),
    }
}
