use crate::{db::DB, WebResult};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskRequest {
    pub name: String,
}

pub async fn create_task_handler(body: TaskRequest, db: DB) -> WebResult<impl Reply> {
    db.create_task(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}
