use serde::{Serialize};
use tokio_postgres::error::Error as TokioError;

#[derive(Serialize)]
pub struct ResponseGetTasks {
    pub message: String,
    pub tasks: String
}