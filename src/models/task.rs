use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string;
use mongodb::bson::DateTime;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskSchema {
    pub _id: String, //ObjectId
    pub name: String,
    pub time_in_seconds: i64,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub initial_time: DateTime,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub end_time: DateTime,
    project: Option<ObjectId>,
    //   createdAt: Date;
    //   updatedAt: Date;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskRequest {
    pub name: String,
    pub time_in_seconds: i64,
    pub initial_time: String,
    pub end_time: String,
    pub project: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskResponse {
    pub _id: String,
    pub name: String,
    pub time_in_seconds: i64,
    pub initial_time: String,
    pub end_time: String,
    pub project: Option<String>,
    //   createdAt: Date;
    //   updatedAt: Date;
}
