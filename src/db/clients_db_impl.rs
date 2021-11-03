use crate::error;
use crate::error::Error::*;
use crate::models::client::{ClientRequest, ClientResponse};
use bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc, Bson};
use mongodb::Collection;
use serde::Serialize;
use serde_json::{Map, Value};

use super::db::{DB, DB_NAME};

impl DB {
    fn get_clients_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection("clients")
    }

    pub fn doc_to_client(&self, doc: &Document) -> Result<ClientResponse, error::Error> {
        let id = doc.get_object_id("_id")?;
        let name = doc.get_str("name")?;
        let created_at = doc.get_datetime("created_at")?;
        let updated_at = doc.get_datetime("updated_at")?;

        let client = ClientResponse {
            _id: id.to_hex(),
            name: name.to_owned(),
            created_at: created_at.to_chrono().to_rfc3339(),
            updated_at: updated_at.to_chrono().to_rfc3339(),
        };

        Ok(client)
    }

    pub async fn find_client(&self, id: &str) -> Result<ClientResponse, error::Error> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let document = self
            .get_clients_collection()
            .find_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        let result = self.doc_to_client(&document.expect("Document not found"))?;

        Ok(result)
    }

    pub async fn create_client(&self, _entry: &ClientRequest) -> Result<(), error::Error> {
        self.get_clients_collection()
            .insert_one(
                doc! {
                "name": _entry.name.clone(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn create_many_clients(
        &self,
        _entry: Vec<mongodb::bson::Document>,
    ) -> Result<(), error::Error> {
        // let serielized_clients = _entry.serialize(serializer);
        // Serialize it to a JSON string.
        // println!("{}", "HELLO THERE");
        // let j = serde_json::to_string_pretty(&_entry).unwrap();
        // println!("{}", j);

        // // let doc: bson::Document = j.to_document();
        // let bson_doc = bson::to_vec(&j).unwrap();

        // let document = bson_doc;

        // let value: Map<String, Value> = serde_json::from_str(&j).unwrap();
        // let document = Document::try_from(value).unwrap();
        // println!("DCOUMENT {:?}", &document);

        self.get_clients_collection()
            .insert_many(_entry, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_client(&self, id: &str) -> Result<(), error::Error> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        self.get_clients_collection()
            .delete_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn delete_all_clients(&self) -> Result<(), error::Error> {
        self.get_clients_collection()
            .delete_many(doc! {}, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }
}
