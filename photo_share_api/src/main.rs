mod handlers;
mod model;

use async_graphql::{EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router};
use handlers::{graphql_handler, graphql_playground};
use model::{Mutation, Query};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
