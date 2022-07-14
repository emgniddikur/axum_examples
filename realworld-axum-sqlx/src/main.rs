use axum::{response::Html, routing::get, Router};
use clap::Parser;
use sqlx::{migrate, postgres::PgPoolOptions};
use std::net::SocketAddr;

use realworld_axum_sqlx::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::parse();
    println!("{}", config.database_url);

    let pool = PgPoolOptions::new().connect(&config.database_url).await?;

    migrate!().run(&pool).await?;

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn root() -> Html<&'static str> {
    Html("Hello, World!")
}
