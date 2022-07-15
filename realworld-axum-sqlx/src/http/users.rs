use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new().route("/api/users", get(list_user))
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
}

async fn list_user() -> Json<Vec<User>> {
    Json(vec![User {
        username: "foo".to_string(),
    }])
}
