use crate::http::ApiContext;
use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/api/users", get(list_user).post(create_user))
        .route("/api/users/:user_id", get(get_user).delete(delete_user))
}

#[derive(Serialize, Deserialize)]
struct User {
    user_id: Uuid,
}

async fn list_user(ctx: Extension<ApiContext>) -> Json<Vec<User>> {
    let users = query_as!(User, r#"select * from "user""#)
        .fetch_all(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();

    Json(users)
}

async fn get_user(ctx: Extension<ApiContext>, Path(id): Path<Uuid>) -> Json<User> {
    let user = query_as!(User, r#"select * from "user" where user_id = $1"#, id)
        .fetch_one(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();

    Json(user)
}

async fn create_user(ctx: Extension<ApiContext>, Json(req): Json<User>) {
    query!(r#"insert into "user" default values"#)
        .execute(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();
}

async fn delete_user(ctx: Extension<ApiContext>, Path(id): Path<Uuid>) {
    query!(r#"delete from "user" where user_id = $1"#, id)
        .execute(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();
}
