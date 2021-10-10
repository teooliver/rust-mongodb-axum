use chrono::DateTime;
use serde::{Deserialize, Serialize};

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct TaskSchema {
    name: String,
    timeInSeconds: u64,
    initialTime: DateTime,
    endTime: DateTime,
    project: MongoDbRef,
}
