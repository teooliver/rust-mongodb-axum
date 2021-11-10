use crate::db::{self, DB};
use crate::error::Error::InvalidIDError;

use crate::models::task::TaskRequest;
use crate::{models::project::ProjectRequest, WebResult};
use fake::{self, Fake};
use mongodb::bson::{doc, oid::ObjectId};
use rand::Rng;
use warp::{http::StatusCode, Reply};

pub const PROJECT_COLORS: [&str; 10] = [
    "#61e294ff",
    "#7bcdbaff",
    "#9799caff",
    "#bd93d8ff",
    "#b47aeaff",
    "#d3d5d4ff",
    "#a2c5acff",
    "#9db5b2ff",
    "#878e99ff",
    "#7f6a93ff",
];

// pub const CLIENT_NAMES = [];
// pub const PROJECT_NAMES = [];
// pub const TASK_NAMES = [];

pub fn generate_clients_data(amount: u8) -> Vec<mongodb::bson::Document> {
    let mut clients: Vec<mongodb::bson::Document> = vec![];

    for _n in 1..amount {
        clients.push(doc! {
            "name": fake::faker::company::en::CompanyName().fake::<String>().to_string(),
            "created_at": chrono::Utc::now().clone(),
            "updated_at": chrono::Utc::now().clone(),
        });
    }

    clients
}

pub fn create_project(clients_ids: Vec<String>) -> ProjectRequest {
    let rng_color_index = rand::thread_rng().gen_range(0..(PROJECT_COLORS.len() - 1));
    let rng_client_index = rand::thread_rng().gen_range(0..(clients_ids.len() - 1));

    let client_id = ObjectId::parse_str(clients_ids[rng_client_index].to_string())
        .map_err(|_| InvalidIDError(clients_ids[rng_client_index].to_owned()))
        .unwrap();

    let new_project = ProjectRequest {
        client: client_id,
        name: fake::faker::company::en::CompanyName().fake(),
        color: PROJECT_COLORS[rng_color_index].to_string(),
        estimate: "".to_string(),
        status: "".to_string(),
    };

    new_project
}

pub fn generate_projects_data(
    amount: u8,
    clients_ids: Vec<String>,
) -> Vec<mongodb::bson::Document> {
    let mut projects: Vec<mongodb::bson::Document> = vec![];
    let project = create_project(clients_ids);

    for _n in 1..amount {
        projects.push(doc! {
            "client": project.client.to_string(),
            "name": project.name.to_string(),
            "color": project.color.to_string(),
            "estimate": project.estimate.to_string(),
            "status": project.status.to_string(),
        });
    }

    projects
}

pub fn create_task(project_ids: Vec<String>) -> TaskRequest {
    let rng_project_index = rand::thread_rng().gen_range(0..(project_ids.len() - 1));

    let project_id = ObjectId::parse_str(project_ids[rng_project_index].to_string())
        .map_err(|_| InvalidIDError(project_ids[rng_project_index].to_owned()))
        .unwrap();

    let new_task = TaskRequest {
        name: fake::faker::company::en::CompanyName().fake(),
        time_in_seconds: 10000,
        initial_time: chrono::Utc::now().clone().to_string(),
        end_time: chrono::Utc::now().clone().to_string(),
        project: Some(project_id.to_hex()),
    };

    // get_all_projects_ids
    new_task
}

pub fn generate_tasks_data(amount: u8, clients_ids: Vec<String>) -> Vec<mongodb::bson::Document> {
    let mut tasks: Vec<mongodb::bson::Document> = vec![];
    let task = create_task(clients_ids);

    for _n in 1..amount {
        tasks.push(doc! {
            "name": task.name.to_string(),
            "time_in_seconds": task.time_in_seconds.to_string(),
            "initial_time": task.initial_time.to_string(),
            "end_time": task.end_time.to_string(),
            "project": Some(task.project.to_owned()),
        });
    }

    tasks
}

pub async fn seed_clients(db: DB) -> WebResult<impl Reply> {
    db.delete_all_clients().await?;
    db.delete_all_projects().await?;
    db.delete_all_tasks().await?;

    db.create_many_clients(generate_clients_data(10)).await?;

    Ok(StatusCode::OK)
}

pub async fn seed_projects(db: DB) -> WebResult<impl Reply> {
    db.delete_all_projects().await?;
    db.delete_all_tasks().await?;

    let client_ids = db.get_all_clients_ids().await?;

    db.create_many_projects(generate_projects_data(10, client_ids))
        .await?;

    Ok(StatusCode::OK)
}

pub async fn seed_tasks(db: DB) -> WebResult<impl Reply> {
    db.delete_all_tasks().await?;

    let projects_ids = db.get_all_projects_ids().await?;

    db.create_many_tasks(generate_tasks_data(10, projects_ids))
        .await?;

    Ok(StatusCode::OK)
}
