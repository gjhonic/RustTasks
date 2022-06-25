use serde::{Serialize};

#[derive(Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub status: i32
}

#[derive(Serialize)]
pub struct ResponseGetTasks {
    pub message: String,
    pub tasks: Vec<Task>
}