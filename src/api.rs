use tokio_postgres::error::Error;
use warp::{reject};
use bb8::Pool;
use tokio_postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;
use crate::{ConnectionPool};
use crate::entitys::{Task, TaskRequest};

#[derive(Debug)]
struct ConnError;

impl reject::Reject for ConnError {}

#[derive(Debug)]
struct DataError;

impl reject::Reject for DataError {}

//Возвращает Таски
pub async fn get_tasks(pool: ConnectionPool) -> Result<impl warp::Reply, warp::Rejection> {

    //Ok(format!("get tasks."))

    println!("[*] Query get_tasks");

    let conn = pool.get().await.map_err(|_| reject::custom(ConnError))?;

    let mut res = String::from("");

    for row in conn
        .query("SELECT id, name, status FROM tasks", &[])
        .await
        .map_err(|_| reject::custom(ConnError))?
    {
        let (task_id, task_name, task_status): (
            Result<i32, Error>,
            Result<String, Error>,
            Result<i32, Error>,
        ) = (row.try_get(0), row.try_get(1), row.try_get(2));

        res.push_str(&format!(
            "id: {} name: {} status: {} \n",
            task_id.map_err(|_| reject::custom(DataError))?,
            task_name.map_err(|_| reject::custom(DataError))?,
            task_status.map_err(|_| reject::custom(DataError))?
        ));
    }

    Ok(res)
}

//Добавляет таск
pub async fn add_task(
    body: TaskRequest,
    pool: ConnectionPool
) -> Result<impl warp::Reply, warp::Rejection> {

    println!("[*] Query add_task");

    let conn = pool.get().await.map_err(|_| reject::custom(ConnError))?;

    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", "tasks");
    let row = conn
            .query_one(query.as_str(), &[&body.name])
            .await
            .map_err(|_| reject::custom(ConnError))?;
            
    Ok(format!("task was added."))
}