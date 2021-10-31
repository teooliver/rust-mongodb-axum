use db::DB;
use std::convert::Infallible;
use warp::{http::Method, Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod error;
mod handler;
mod models;
mod project_db_impl;

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let cors = warp::cors().allow_any_origin();
    // .allow_header("content-type")
    // .allow_methods(&[Method::PUT, Method::DELETE]);

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
            .and(warp::path("dangerously-delete-all-tasks"))
            .and(with_db(db.clone()))
            .and_then(handler::delete_all_tasks_handler))
        .or(tasks
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_task_handler));

    let projects = warp::path("projects");

    let projects_routes = projects
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_project_handler)
        .or(projects
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::fetch_project_handler))
        .or(projects
            .and(warp::delete())
            .and(warp::path("dangerously-delete-all-projects"))
            .and(with_db(db.clone()))
            .and_then(handler::delete_all_projects_handler))
        .or(projects
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_project_handler));

    let routes = task_routes
        .or(projects_routes)
        .with(cors)
        .recover(error::handle_rejection);

    println!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
