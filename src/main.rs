use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use warp::{Filter};
mod api;
use std::fs;
use std::env;
mod entitys;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

const CONN: &str = "postgresql://rust:1111@postgres:5432/rusttasks";

fn with_pool(
    pool: ConnectionPool,
) -> impl Filter<Extract = (ConnectionPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

//Добавляет таск
async fn index() -> Result<impl warp::Reply, warp::Rejection> {

    println!("[*] Query index");

    let contents = fs::read_to_string("static/pages/index.html")
        .expect("Should have been able to read the file");
            
    Ok(warp::reply::html(contents))
}

#[tokio::main]
async fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let manager = PostgresConnectionManager::new_from_stringlike(CONN, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let route_index = warp::path("index")
        .and_then(index);

    let route_index_empty = warp::path::end()
        .and_then(index);

    let route_get_tasks = warp::path("get-tasks")
        .and(with_pool(pool.clone()))
        .and_then(api::get_tasks);

    let route_create_task = warp::path("add-task")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_pool(pool))
        .and_then(api::add_task);

    let route_static = warp::path("static").and(warp::fs::dir("static"));

    let routes = route_index
        .or(route_index_empty)
        .or(route_get_tasks)
        .or(route_create_task)
        .or(route_static);

    println!("Server success started: 0.0.0.0:3030");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}