use axum::{handler::get, Router};
use mongodb::{
    bson::{self, doc, Document},
    Client, Collection,
};
use thiserror::Error;
// Handler that immediately returns an empty `200 OK` response.
async fn unit_handler() {}

// Handler that immediately returns an empty `200 OK` response with a plain
// text body.
async fn string_handler() -> String {
    "Hello, World!!!!!!".to_string()
}

// async fn create_task()->

#[derive(Error, Debug)]
pub enum Error {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("invalid id used: {0}")]
    InvalidIDError(String),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = Client::with_uri_str("mongodb://127.0.0.1:27017").await?;
    let time_tracker_base_db = client.database("time-tracker-base");

    let tasks: Collection<Document> = time_tracker_base_db.collection("tasks");

    tasks
        .insert_one(
            doc! {
                "hello": "hello there"
            },
            None,
        )
        .await?;
    // build our application with a single route
    let app = Router::new().route("/", get(string_handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Runnin on localhost:3000");
    Ok(())
}
