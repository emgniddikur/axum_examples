use crate::http::ApiContext;
use axum::{extract::Extension, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::query_scalar;

pub fn router() -> Router {
    Router::new().route("/api/users", get(list_user).post(create_user))
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

async fn create_user(ctx: Extension<ApiContext>, Json(req): Json<User>) {
    query_scalar!(r#"insert into "user" default values"#)
        .fetch_one(&ctx.pool)
        .await
        .unwrap();
}
