use crate::http::ApiContext;
use axum::{
    extract::{Extension, Path},
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use sqlx::{query, query_scalar};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/articles", post(create_article))
        .route("/articles/:id/favorite", post(favorite_article))
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FavoriteArticle {
    user_id: Uuid,
}

async fn favorite_article(
    ctx: Extension<ApiContext>,
    Path(id): Path<Uuid>,
    Json(req): Json<FavoriteArticle>,
) -> Json<Uuid> {
    let article_id = query_scalar!(
        "
            with selected_article as (
                select article_id from articles where article_id = $1
            ),
            inserted_favorite as (
                insert into article_favorites (user_id, article_id)
                select $2, article_id
                from selected_article
                on conflict do nothing
            )
            select article_id from selected_article
        ",
        id,
        req.user_id,
    )
    .fetch_optional(&ctx.pool)
    .await
    .unwrap()
    .unwrap();

    Json(article_id)
}
