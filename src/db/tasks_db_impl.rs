use crate::models::task::{GroupedTasks, TaskAfterGrouped, TaskRequest, TaskResponse};
use crate::{error::Error::*, Result};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson;
use mongodb::bson::{doc, document::Document, oid::ObjectId};
use mongodb::Collection;

use super::{DB, DB_NAME};

impl DB {
    fn get_tasks_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection("tasks")
    }

    fn doc_to_task(&self, doc: &Document) -> Result<TaskResponse> {
        // println!("{:?}", doc);
        let id = doc.get_object_id("_id")?;
        let name = doc.get_str("name")?;
        let time_in_seconds = doc.get_i32("time_in_seconds")?;
        let initial_time = doc.get_datetime("initial_time")?;
        let end_time = doc.get_datetime("end_time")?;
        let project = doc.get_object_id("project")?;
        let created_at = doc.get_datetime("created_at")?;
        let updated_at = doc.get_datetime("updated_at")?;

        // let match_project: Option<ObjectId> = match project {
        //     project => Some(project),
        //     None => None,
        // };

        // if project.is_none() {
        //     // return error::Err(warp::reject::not_found());
        //     return Err(ObjNotFound);
        // }

        let task = TaskResponse {
            _id: id.to_hex(),
            name: name.to_owned(),
            time_in_seconds: time_in_seconds.to_owned(),
            // initial_time: initial_time.to_string(),
            initial_time: initial_time.to_chrono().to_rfc3339(),
            // end_time: end_time.to_string(),
            end_time: end_time.to_chrono().to_rfc3339(),
            project: Some(project.to_hex()),
            created_at: created_at.to_chrono().to_rfc3339(),
            updated_at: updated_at.to_chrono().to_rfc3339(),
        };

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

    pub async fn get_tasks_grouped_by_date(&self) -> Result<GroupedTasks> {
        let lookup_projects = doc! {
            "$lookup": {
                "from": "projects",
                "localField": "project",
                "foreignField": "_id",
                "as": "project",
            }
        };
        let lookup_clients = doc! {
            "$lookup": {
              "from": "clients",
              "localField": "project.client",
              "foreignField": "_id",
              "as": "client",
            }
        };

        let project = doc! {
              "$project": {
                    "_id": "$_id",
                    "name": "$name",
                    "timeInSeconds": "$timeInSeconds",
                    "initialTime": "$initialTime",
                    "endTime": "$endTime",
                    "project": "{ $arrayElemAt: ['$project.name', 0] }",
                    "projectColor": "{ $arrayElemAt: ['$project.color', 0] }",
                    "client": "{ $arrayElemAt: ['$client.name', 0] }",
                },
        };

        let group = doc! {
            "$group": {
                "_id": { "$dateToString": { "format": "%Y-%m-%d", "date": "$initialTime" } },
                "tasks": { "$push": "$$ROOT" },
                "totalTime": {
                    "$sum":{
                        "$divide": [{ "$subtract": ["$endTime", "$initialTime"] }, 1000],
                    },
                },
            },
        };

        let sort = doc! {
             "$sort": {
                "_id": -1,
            },
        };

        let pipeline = vec![lookup_projects, lookup_clients, project, group, sort];

        let mut cursor = self
            .get_tasks_collection()
            .aggregate(pipeline, None)
            .await?;

        let mut tasks_vec: Vec<TaskAfterGrouped>;
        while let Some(doc) = cursor.next().await {
            let doc_real = doc.unwrap();
            let tasks = doc_real.get_array("tasks")?;

            println!("{:?}", tasks);

            // results.push(self.doc_project_grouped_by_client(&doc?)?);
        }

        // println!("{:?}", results);

        // Ok(results)

        todo!()
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

        // println!("{:?}", document);

        if document.is_none() {
            // return error::Err(warp::reject::not_found());
            return Err(ObjNotFound);
        }

        let result = self.doc_to_task(&document.unwrap())?;

        Ok(result)
    }

    pub async fn create_task(&self, _entry: &TaskRequest) -> Result<()> {
        let chrono_dt: chrono::DateTime<Utc> = _entry.initial_time.parse().unwrap();
        let initial_time: bson::DateTime = chrono_dt.into();
        let chrono_endtime: chrono::DateTime<Utc> = _entry.end_time.parse().unwrap();
        let end_time: bson::DateTime = chrono_endtime.into();
        let project: Option<ObjectId> = _entry.project.clone();

        self.get_tasks_collection()
            .insert_one(
                doc! {
                "name": _entry.name.clone(),
                "time_in_seconds": _entry.time_in_seconds.clone(),
                "initial_time": initial_time.clone(),
                "end_time": end_time.clone(),
                "project": project,
                "created_at": chrono::Utc::now().clone(),
                "updated_at": chrono::Utc::now().clone(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn edit_task(&self, id: &str, _entry: &TaskRequest) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let chrono_dt: chrono::DateTime<Utc> = _entry.initial_time.parse().unwrap();
        let initial_time: bson::DateTime = chrono_dt.into();
        let chrono_endtime: chrono::DateTime<Utc> = _entry.end_time.parse().unwrap();
        let end_time: bson::DateTime = chrono_endtime.into();
        let project: Option<ObjectId> = _entry.project.clone();

        let query = doc! {
            "_id": oid,
        };

        let doc = doc! {
            "$set": {
                "name": _entry.name.clone(),
                "time_in_seconds": _entry.time_in_seconds.clone(),
                "initial_time": initial_time.clone(),
                "end_time": end_time.clone(),
                "project": project,
                "updated_at": chrono::Utc::now().clone(),
                }
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

    pub async fn delete_task(&self, id: &str) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        self.get_tasks_collection()
            .delete_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn create_many_tasks(&self, _entry: Vec<mongodb::bson::Document>) -> Result<()> {
        self.get_tasks_collection()
            .insert_many(_entry, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }
}
