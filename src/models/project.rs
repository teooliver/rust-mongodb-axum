use mongodb::bson::oid::ObjectId;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectSchema {
    pub _id: String, //ObjectId
    pub name: String,
    pub color: String,
    pub estimate: String,
    pub status: String,
    //   createdAt: Date;
    //   updatedAt: Date;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectRequest {
    pub client: String,
    pub name: String,
    pub color: String,
    pub estimate: String,
    pub status: String,
    //   createdAt: Date;
    //   updatedAt: Date;
}
