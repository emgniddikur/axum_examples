use crate::http::ApiContext;
use axum::{extract::Extension, routing::post, Json, Router};
use serde::Deserialize;
use sqlx::query;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/articles", post(create_article))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateArticle {
    user_id: Uuid,
    slug: String,
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
}

async fn create_article(ctx: Extension<ApiContext>, Json(req): Json<CreateArticle>) {
    query!(
        "insert into articles (user_id, slug, title, description, body, tag_list) values ($1, $2, $3, $4, $5, $6)",
        req.user_id,
        req.slug,
        req.title,
        req.description,
        req.body,
        &req.tag_list,
    )
    .execute(&ctx.pool)
    .await
    .unwrap();
}
