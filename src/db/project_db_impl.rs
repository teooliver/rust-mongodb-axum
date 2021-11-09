use crate::models::project::{ProjectRequest, ProjectSchema};
use crate::{error::Error::*, Result};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::Collection;

use super::{DB, DB_NAME};

impl DB {
    fn get_projects_collection(&self) -> Collection<Document> {
        self.client.database(DB_NAME).collection("projects")
    }

    pub fn doc_to_project(&self, doc: &Document) -> Result<ProjectSchema> {
        let id = doc.get_object_id("_id")?;
        let name = doc.get_str("name")?;
        let color = doc.get_str("color")?;
        let estimate = doc.get_str("estimate")?;
        let status = doc.get_str("status")?;

        let project = ProjectSchema {
            _id: id.to_hex(),
            name: name.to_owned(),
            color: color.to_owned(),
            estimate: estimate.to_owned(),
            status: status.to_owned(),
        };

        Ok(project)
    }

    pub async fn find_project(&self, id: &str) -> Result<ProjectSchema> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let document = self
            .get_projects_collection()
            .find_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        // if document == None {
        //     // return error::Err(warp::reject::not_found());
        //     return Err(ObjNotFound);
        // }

        let result = self.doc_to_project(&document.expect("Document not found"))?;

        Ok(result)
    }

    // pub fn get_projects_grouped_by_client(&self) {

    //     let pipeline = doc! {
    //         [
    //   {
    //     $lookup: {
    //       from: 'clients',
    //       localField: 'client',
    //       foreignField: '_id',
    //       as: 'clientName',
    //     },
    //   },
    //   {
    //     $sort: {
    //       updatedAt: -1,
    //     },
    //   },
    //   {
    //     $project: {
    //       _id: '$_id',
    //       name: '$name',
    //       color: '$color',
    //       clientName: { $arrayElemAt: ['$clientName.name', 0] },
    //       estimate: '$estimate',
    //       status: '$status',
    //       subprojects: '$subprojects',
    //     },
    //   },
    //   {
    //     $group: {
    //       _id: '$clientName',
    //       projects: { $push: '$$ROOT' },
    //     },
    //   },
    // ]
    //     };

    //     self.get_projects_collection().aggregate(pipeline, None);
    // };

    pub async fn create_project(&self, _entry: &ProjectRequest) -> Result<()> {
        self.get_projects_collection()
            .insert_one(
                doc! {
                "name": _entry.name.clone(),
                "color": _entry.color.clone(),
                "estimate": _entry.estimate.clone(),
                "status": _entry.status.clone(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn delete_project(&self, id: &str) -> Result<()> {
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        self.get_projects_collection()
            .delete_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn delete_all_projects(&self) -> Result<()> {
        self.get_projects_collection()
            .delete_many(doc! {}, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(())
    }

    pub async fn create_many_projects(&self, _entry: Vec<mongodb::bson::Document>) -> Result<()> {
        self.get_projects_collection()
            .insert_many(_entry, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }
}
