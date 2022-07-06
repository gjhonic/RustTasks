use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub status: i32,
}

#[derive(Deserialize)]
pub struct TaskRequest {
    pub name: String,
}