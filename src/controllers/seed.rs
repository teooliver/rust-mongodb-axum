use fake::{self, Fake};
use mongodb::{bson::doc, Client};

use crate::{db::db::DB, models::client::ClientRequest};

pub const PROJECT_COLORS: [&str; 10] = [
    "#61e294ff",
    "#7bcdbaff",
    "#9799caff",
    "#bd93d8ff",
    "#b47aeaff",
    "#d3d5d4ff",
    "#a2c5acff",
    "#9db5b2ff",
    "#878e99ff",
    "#7f6a93ff",
];

// pub const CLIENT_NAMES = [];
// pub const PROJECT_NAMES = [];
// pub const TASK_NAMES = [];

fn create_client() -> ClientRequest {
    let new_client = ClientRequest {
        name: fake::faker::company::en::CompanyName().fake(),
    };

    new_client
}

pub fn generate_clients_data(amount: u8) -> Vec<mongodb::bson::Document> {
    let mut clients: Vec<mongodb::bson::Document> = vec![];

    for _n in 1..amount {
        clients.push(doc! {"name": create_client().name});
    }

    clients
}

pub async fn seed_clients(db: &DB) {
    // db.delete_all_clients().await;
    // db.delete_all_projects().await;
    // db.delete_all_tasks().await;

    db.create_many_clients(generate_clients_data(10)).await;

    //  try {
    //   await Client.deleteMany();
    //   await Project.deleteMany();
    //   await Task.deleteMany();

    //   // @ts-ignore Monggose not playing nice with Model.create and typescript
    //   const savedClients = await Client.create(clientsList);

    //   res.status(201).json(savedClients);
    // } catch (error) {
    //   res.status(409).json(error);
    // }
}

// pub fn seed_clients() {
//     let clients_list = generate_clients_data(5);
// }
