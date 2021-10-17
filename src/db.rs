use crate::{error::Error::*, handler::TaskRequest, Book, Result};
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
}
