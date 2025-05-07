mod db;
mod models;
mod repositories;
mod controllers;
mod schema;

use axum::{
    Router, routing::{get, post, put, delete},
};
use controllers::{contact_controller::{create_contact, list_contacts, delete_contact}, post_controller::{create_post, delete_post, get_post, list_posts, update_post}};
use db::establish_connection;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let pool = establish_connection();

    let app = Router::new()
        .route("/posts", get(list_posts))
        .route("/posts", post(create_post))
        .route("/posts/{id}", get(get_post))
        .route("/posts/{id}", put(update_post))
        .route("/posts/{id}", delete(delete_post))
        .route("/contact", post(create_contact))
        .route("/contact", get(list_contacts))
        .route("/contact/{id}", delete(delete_contact))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
