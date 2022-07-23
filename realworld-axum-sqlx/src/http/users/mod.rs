pub mod follows;

use crate::http::{
    error::{Error, ResultExt},
    ApiContext, Result,
};
use axum::{
    extract::{Extension, Path},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, query_scalar};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/users", get(list_user).post(create_user))
        .route(
            "/users/:user_id",
            get(get_user).patch(update_user).delete(delete_user),
        )
}

#[derive(Serialize, Deserialize)]
struct User {
    user_id: Uuid,
    username: String,
}

async fn list_user(ctx: Extension<ApiContext>) -> Json<Vec<User>> {
    let users = query_as!(User, "select * from users")
        .fetch_all(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();

    Json(users)
}

async fn get_user(ctx: Extension<ApiContext>, Path(id): Path<Uuid>) -> Json<User> {
    let user = query_as!(User, "select * from users where user_id = $1", id)
        .fetch_one(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();

    Json(user)
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

async fn create_user(
    ctx: Extension<ApiContext>,
    Json(req): Json<CreateUser>,
) -> Result<Json<User>> {
    let user_id = query_scalar!(
        "insert into users (username) values ($1) returning user_id",
        req.username
    )
    .fetch_one(&ctx.pool)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })?;

    Ok(Json(User {
        user_id,
        username: req.username,
    }))
}

#[derive(Deserialize)]
struct UpdateUser {
    username: String,
}

async fn update_user(
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUser>,
) {
    query!(
        "update users set username = $2 where user_id = $1",
        id,
        req.username
    )
    .execute(&ctx.pool)
    .await
    // TODO: error handling
    .unwrap();
}

async fn delete_user(ctx: Extension<ApiContext>, Path(id): Path<Uuid>) {
    query!("delete from users where user_id = $1", id)
        .execute(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();
}
