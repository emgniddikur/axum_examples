use axum::{routing::post, Router};

pub fn router() -> Router {
    Router::new().route("/articles", post(create_article))
}

async fn create_article() {
    println!("create_article");
}
