use chrono;
use chrono::Utc;
use db::DB;
use mongodb::bson;
use std::convert::Infallible;
use warp::{Filter, Rejection};
type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod error;
mod handler;
mod task;

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let chrono_dt: chrono::DateTime<Utc> = "2021-10-19T20:25:17.734Z".parse().unwrap();
    let initial_time: bson::DateTime = chrono_dt.into();

    println!("{:?}", initial_time.to_string());

    // TODO: add "api/v1" to all routes
    let tasks = warp::path("tasks");

    let task_routes = tasks
        .and(warp::path("all"))
        .and(with_db(db.clone()))
        .and_then(handler::fetch_all_tasks_handler)
        .or(tasks
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::fetch_task_handler))
        .or(tasks
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::create_task_handler))
        .or(tasks
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_task_handler))
        .or(tasks
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handler::delete_all_tasks_handler));

    let routes = task_routes.recover(error::handle_rejection);

    println!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
