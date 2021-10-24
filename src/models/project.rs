use mongodb::bson::oid::ObjectId;
// use mongodb::bson::oid::ObjectId;
use serde::{self, Deserialize, Serialize};

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectSchema {
    pub _id: ObjectId, //ObjectId
    pub name: String,
    pub color: String,
    pub estimate: String,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectRequest {
    pub name: String,
    pub color: String,
    pub estimate: String,
    pub status: String,
}
