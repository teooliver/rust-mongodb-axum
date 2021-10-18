use crate::{error::Error::*, handler::TaskRequest, Result, Task};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
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

    fn doc_to_task(&self, doc: &Document) -> Result<Task> {
        let id = doc.get_object_id("_id")?;
        let name = doc.get_str("name")?;

        let book = Task {
            id: id.to_hex(),
            name: name.to_owned(),
        };
        Ok(book)
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let mut cursor = self
            .get_tasks_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Task> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_task(&doc?)?);
        }
        Ok(result)
    }

    pub async fn create_task(self, _entry: &TaskRequest) -> Result<()> {
        self.get_tasks_collection()
            .insert_one(
                doc! {
                    "name": "Another one 333333"
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
            "name": _entry.name.clone(),
        };

        println!("{}", query);
        println!("{:?}", _entry);

        // self.get_tasks_collection().find_one_and_update(filter, update, options)

        let x = self
            .get_tasks_collection()
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;

        println!("{:?}", x);
        Ok(())
    }
    pub async fn delete_all_tasks(self) -> Result<()> {
        self.get_tasks_collection()
            .delete_many(doc! {}, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }
}
