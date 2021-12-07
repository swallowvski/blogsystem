use std::sync::Mutex;

use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    AddExtensionLayer, Json, Router,
};

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Request, Response, Schema,
};

use once_cell::sync::Lazy;

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

type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<ApiSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

static PHOTOS: Lazy<Mutex<Vec<Photo>>> = Lazy::new(|| Mutex::new(vec![]));

struct Photo {
    name: String,
    description: String,
}

struct Query;

#[Object]
impl Query {
    async fn total_photos(&self) -> usize {
        PHOTOS.lock().unwrap().len()
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn post_photo(&self, name: String, description: String) -> bool {
        let photo = Photo { name, description };
        PHOTOS.lock().unwrap().push(photo);
        true
    }
}
