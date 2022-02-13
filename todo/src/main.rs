use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
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
        .route("/todos/:id", patch(update))
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

#[derive(Deserialize)]
struct UpdateTodo {
    text: String,
    completed: bool,
}

async fn update(
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTodo>,
    Extension(db): Extension<Db>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    todo.text = input.text;
    todo.completed = input.completed;

    db.write().unwrap().insert(todo.id, todo.clone());

    Ok(Json(todo))
}
