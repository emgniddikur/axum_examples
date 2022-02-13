use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tower_http::add_extension::AddExtensionLayer;
use uuid::Uuid;

mod root;

#[tokio::main]
async fn main() {
    let db = Db::default();

    let app = Router::new()
        .route("/", get(root::root))
        .route("/todos", get(index).post(create))
        .layer(AddExtensionLayer::new(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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

#[derive(Deserialize)]
struct CreateTodo {
    text: String,
}

async fn create(Json(input): Json<CreateTodo>, Extension(db): Extension<Db>) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    db.write().unwrap().insert(todo.id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}
