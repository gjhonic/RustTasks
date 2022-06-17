use serde::{Serialize};

#[derive(Serialize)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub status: u64
}

// Ответ по API getTask
#[derive(Serialize)]
pub struct ResponseTaskJson {
    status: bool,
    message: String,
    task: Task
}

// Ответ по API getTasks
#[derive(Serialize)]
pub struct ResponseTasksJson {
    status: bool,
    message: String,
    tasks: Vec<Task>
}