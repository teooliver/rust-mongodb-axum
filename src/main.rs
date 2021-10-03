use axum::{handler::get, Router};
// Handler that immediately returns an empty `200 OK` response.
async fn unit_handler() {}

// Handler that immediately returns an empty `200 OK` response with a plain
// text body.
async fn string_handler() -> String {
    "Hello, World!!!!!!".to_string()
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(string_handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
