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
                data: Some(serde_json::to_value(post).expect("Failed to serialize post")),
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

pub async fn update_post(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<NewPost>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return (axum::http::StatusCode::BAD_REQUEST, {
            let errs = e.field_errors().iter().map(|(field, errors)| {
                (field.to_string(), errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>())
            }).collect::<serde_json::Value>();
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
    match repo.find_by_id(id) {
        Ok(_) => {},
        Err(_) => return (axum::http::StatusCode::NOT_FOUND, {
            let reponse = ApiResponse {
                status: 404,
                message: "Post not found".to_string(),
                data: None::<()>,
            };
            Json(reponse)
        }).into_response(),
    }
    let update_post = NewPost{
        id: Some(id),
        title: payload.title,
        body: payload.body,
        published: payload.published,
    };
    match repo.update(update_post) {
        Ok(post) => (axum::http::StatusCode::OK, {
            let response = ApiResponse {
                status: 200,
                message: "OK".to_string(),
                data: Some(post),
            };
            Json(response)
        }).into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to update post").into_response(),
    }
}
