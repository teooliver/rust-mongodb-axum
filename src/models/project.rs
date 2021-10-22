// use mongodb::bson::oid::ObjectId;
use serde::{self, Deserialize, Serialize};

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectSchema {
    pub _id: String, //ObjectId
    pub name: String,
    pub color: String,
    pub estimate: String,
    pub status: String,
}
