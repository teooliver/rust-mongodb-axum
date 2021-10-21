// use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskSchema {
    pub _id: String, //ObjectId
    pub name: String,
    pub time_in_seconds: i64,
    pub initial_time: DateTime,
    pub end_time: DateTime,
    // project: MongoDbRef,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskRequest {
    pub name: String,
    pub time_in_seconds: i64,
    pub initial_time: String,
    pub end_time: String,
    // project: MongoDbRef,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskResponse {
    pub _id: String,
    pub name: String,
    pub time_in_seconds: i64,
    pub initial_time: String,
    pub end_time: String,
    // project: MongoDbRef,
}
