mod todo;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    AddExtensionLayer, Json, Router,
};
use std::net::SocketAddr;
use todo::{Mutation, QueryRoot, TodoSchema};

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphql_handler(schema: Extension<TodoSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
