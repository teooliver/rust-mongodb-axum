#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}


impl DB {
  pub async fn init() -> Result<Self>{
    let mut client = Client::with_uri_str("mongodb://127.0.0.1:27017").await?;
    let time_tracker_base_db = client.database("time-tracker-base");
    let tasks: Collection<Document> = time_tracker_base_db.collection("tasks");
  }
}