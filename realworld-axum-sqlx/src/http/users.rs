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
    let users = query_as!(User, r#"select * from users"#)
        .fetch_all(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();

    Json(users)
}

async fn get_user(ctx: Extension<ApiContext>, Path(id): Path<Uuid>) -> Json<User> {
    let user = query_as!(User, r#"select * from users where user_id = $1"#, id)
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

async fn create_user(ctx: Extension<ApiContext>, Json(req): Json<CreateUser>) {
    query!(r#"insert into users (username) values ($1)"#, req.username)
        .execute(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();
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
        r#"update users set username = $2 where user_id = $1"#,
        id,
        req.username
    )
    .execute(&ctx.pool)
    .await
    // TODO: error handling
    .unwrap();
}

async fn delete_user(ctx: Extension<ApiContext>, Path(id): Path<Uuid>) {
    query!(r#"delete from users where user_id = $1"#, id)
        .execute(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();
}
