use std::str::FromStr;

use crate::task::TaskSchema;
use crate::task::{TaskRequest, TaskResponse};
use crate::{error::Error::*, Result};
use chrono::prelude::*;
// use chrono::{DateTime, Utc};
use futures::StreamExt;
use mongodb::bson;
use mongodb::bson::{doc, document::Document, oid::ObjectId, DateTime};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "time-tracker-base";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("time-tracker-base".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    fn get_tasks_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection("tasks")
    }

    fn doc_to_task(&self, doc: &Document) -> Result<TaskResponse> {
        let id = doc.get_object_id("_id")?;
        let name = doc.get_str("name")?;
        let time_in_seconds = doc.get_i64("time_in_seconds")?;
        // let initial_time = doc.get_str("initial_time")?;
        // let end_time = doc.get_str("end_time")?;

        // let chrono_dt: chrono::DateTime<Utc> = _entry.initial_time.parse().unwrap();
        let initial_time = doc.get_datetime("initial_time")?;
        let end_time = doc.get_datetime("end_time")?;
        println!("GOT HERE {:?}", initial_time);

        // let initial_time_dt: chrono::DateTime<Utc> = doc.get_datetime("initial_time")?;
        // let initial_time: bson::DateTime = initial_time_dt.into();
        // println!("INITIAL ==> {:?}", initial_time);

        // let end_time_dt: chrono::DateTime<Utc> = doc.get_datetime("end_time")?.parse().unwrap();
        // let end_time: bson::DateTime = end_time_dt.into();

        // println!("{:?}", initial_time);

        let task = TaskResponse {
            _id: id.to_hex(),
            name: name.to_owned(),
            time_in_seconds: time_in_seconds.to_owned(),
            initial_time: initial_time.to_string(),
            end_time: end_time.to_string(),
        };

        println!("TASK {:?}", task);
        Ok(task)
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<TaskResponse>> {
        let mut cursor = self
            .get_tasks_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<TaskResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_task(&doc?)?);
        }
        Ok(result)
    }

    pub async fn find_task(&self, id: &str) -> Result<TaskResponse> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let document = self
            .get_tasks_collection()
            .find_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        // println!("Document {:?}", document);

        let result = self.doc_to_task(&document.unwrap())?;

        // println!("Result {:?}", result);
        Ok(result)
    }

    pub async fn create_task(&self, _entry: &TaskRequest) -> Result<()> {
        let chrono_dt: chrono::DateTime<Utc> = _entry.initial_time.parse().unwrap();
        let initial_time: bson::DateTime = chrono_dt.into();
        let chrono_endtime: chrono::DateTime<Utc> = _entry.end_time.parse().unwrap();
        let end_time: bson::DateTime = chrono_endtime.into();

        self.get_tasks_collection()
            .insert_one(
                doc! {
                "name": _entry.name.clone(),
                "time_in_seconds": _entry.time_in_seconds.clone(),
                "initial_time": initial_time.clone(),
                "end_time": end_time.clone(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }
    pub async fn edit_task(&self, id: &str, _entry: &TaskRequest) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        println!("{}", oid);

        let query = doc! {
            "_id": oid,
        };

        let doc = doc! {
            "$set": { "name": _entry.name.clone() }
        };

        self.get_tasks_collection()
            .find_one_and_update(query, doc, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }
    pub async fn delete_all_tasks(&self) -> Result<()> {
        self.get_tasks_collection()
            .delete_many(doc! {}, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }
}
