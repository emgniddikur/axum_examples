use axum::{
    extract::Extension, response::IntoResponse, routing::get, AddExtensionLayer, Json, Router,
};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::net::SocketAddr;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app = Router::new()
        .route("/todos", get(index))
        .layer(AddExtensionLayer::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone, Serialize, FromRow)]
struct Todo {
    id: Uuid,
    name: String,
}

async fn index(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let todos = sqlx::query_as::<_, Todo>("select * from todos")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(todos)
}
