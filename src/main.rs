mod db;
mod models;
mod repositories;
mod controllers;
mod schema;

use axum::{
    Router, routing::{get, post},
};
use controllers::post_controller::{list_posts, get_post, create_post};
use db::establish_connection;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let pool = establish_connection();

    let app = Router::new()
        .route("/posts", get(list_posts))
        .route("/posts", post(create_post))
        .route("/posts/{id}", get(get_post))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
