#[allow(dead_code)]
mod controllers;
mod db;
mod error;
mod models;

use std::convert::Infallible;
use warp::{http::Method, Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

use crate::controllers::{clients, projects, seed, tasks};
use crate::db::db::DB;

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;
    // seed::generate_clients_data(10);
    // seed::seed_clients(&db).await;
    // seed::seed_projects(&db).await;

    // db.get_all_clients_ids().await?;

    let cors = warp::cors().allow_any_origin();
    // .allow_header("content-type")
    // .allow_methods(&[Method::PUT, Method::DELETE]);

    // TODO: add "api/v1" to all routes
    let tasks = warp::path("tasks");

    let task_routes = tasks
        .and(warp::path("all"))
        .and(with_db(db.clone()))
        .and_then(tasks::fetch_all_tasks_handler)
        .or(tasks
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(tasks::fetch_task_handler))
        .or(tasks
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(tasks::create_task_handler))
        .or(tasks
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(tasks::edit_task_handler))
        .or(tasks
            .and(warp::delete())
            .and(warp::path("dangerously-delete-all-tasks"))
            .and(with_db(db.clone()))
            .and_then(tasks::delete_all_tasks_handler))
        .or(tasks
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(tasks::delete_task_handler));

    let projects = warp::path("projects");

    let projects_routes = projects
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(projects::create_project_handler)
        .or(projects
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(projects::fetch_project_handler))
        .or(projects
            .and(warp::delete())
            .and(warp::path("dangerously-delete-all-projects"))
            .and(with_db(db.clone()))
            .and_then(projects::delete_all_projects_handler))
        .or(projects
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(projects::delete_project_handler));

    let clients = warp::path("clients");
    let client_routes = clients
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(clients::fetch_client_handler);

    let routes = task_routes
        .or(projects_routes)
        .or(client_routes)
        .with(cors)
        .recover(error::handle_rejection);

    println!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
