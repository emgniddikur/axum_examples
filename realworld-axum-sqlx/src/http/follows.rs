use crate::http::ApiContext;
use axum::{
    extract::{Extension, Path},
    routing::post,
    Router,
};
use sqlx::query;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route(
        "/users/:user_id/follow/:followed_user_id",
        post(follow_user).delete(unfollow_user),
    )
}

async fn follow_user(
    ctx: Extension<ApiContext>,
    Path((user_id, followed_user_id)): Path<(Uuid, Uuid)>,
) {
    let user = query!("select user_id from users where user_id = $1", user_id)
        .fetch_one(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();
    query!(
        "insert into follows (following_user_id, followed_user_id) values ($1, $2)",
        user.user_id,
        followed_user_id
    )
    .execute(&ctx.pool)
    .await
    // TODO: error handling
    .unwrap();
}

async fn unfollow_user(
    ctx: Extension<ApiContext>,
    Path((user_id, followed_user_id)): Path<(Uuid, Uuid)>,
) {
    let user = query!("select user_id from users where user_id = $1", user_id)
        .fetch_one(&ctx.pool)
        .await
        // TODO: error handling
        .unwrap();
    query!(
        "delete from follows where following_user_id = $1 and followed_user_id = $2",
        user.user_id,
        followed_user_id
    )
    .execute(&ctx.pool)
    .await
    // TODO: error handling
    .unwrap();
}
