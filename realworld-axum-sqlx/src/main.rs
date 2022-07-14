use axum::{response::Html, routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;

use realworld_axum_sqlx::config::Config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::parse();
    println!("{}", config.database_url);

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html("Hello, World!")
}
