mod schema;

use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    AddExtensionLayer, Json, Router,
};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Enum, InputObject, Object, Request, Response, Schema, SimpleObject,
};

use crate::schema::model::{Mutation, Query};

type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<ApiSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, Mutation, EmptySubscription);

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    println!("Playground: http://localhost:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
