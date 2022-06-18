use serde::{Serialize};

#[derive(Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub status: i32
}

#[derive(Serialize)]
pub struct ResponseTaskJson {
    pub status: String,
    pub message: String,
    pub task: Task
}

#[derive(Serialize)]
pub struct ResponseTasksJson {
    pub status: String,
    pub message: String,
    pub tasks: Vec<Task>
}