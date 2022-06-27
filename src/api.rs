use tokio_postgres::error::Error;
use warp::{reject};
use bb8::Pool;
use tokio_postgres::NoTls;
use bb8_postgres::PostgresConnectionManager;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Debug)]
struct ConnError;

impl reject::Reject for ConnError {}

#[derive(Debug)]
struct DataError;

impl reject::Reject for DataError {}

//Возвращает Таски
pub async fn get_tasks(pool: ConnectionPool) -> Result<impl warp::Reply, warp::Rejection> {
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
            " | {} | {} | {} |\n",
            task_id.map_err(|_| reject::custom(DataError))?,
            task_name.map_err(|_| reject::custom(DataError))?,
            task_status.map_err(|_| reject::custom(DataError))?
        ));
    }

    Ok(res)
}

//Добавляет таск
pub async fn add_task(
    pool: ConnectionPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    // let conn = pool.get().await.map_err(|_| reject::custom(ConnError))?;

    // conn.execute("INSERT INTO log (log_text) VALUES ($1)", &[&path.as_str()])
    //     .await
    //     .map_err(|_| reject::custom(ConnError))?;

    return Ok(format!("Log was added."));
}