use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tower_http::add_extension::AddExtensionLayer;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let db = Db::default();

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(index))
        .layer(AddExtensionLayer::new(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("Hello, World!")
}

#[derive(Clone, Serialize)]
struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

async fn index(Extension(db): Extension<Db>) -> impl IntoResponse {
    let todos = db.read().unwrap();

    let todos = todos.values().cloned().collect::<Vec<_>>();

    Json(todos)
}
