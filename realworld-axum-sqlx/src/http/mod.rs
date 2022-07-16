mod users;

use crate::config::Config;
use anyhow::Context;
use axum::{extract::Extension, Router};
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    pool: PgPool,
}

pub async fn serve(config: Config, pool: PgPool) -> anyhow::Result<()> {
    let app = api_router().layer(Extension(ApiContext {
        config: Arc::new(config),
        pool,
    }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("")
}

fn api_router() -> Router {
    users::router().merge(users::follows::router())
}
