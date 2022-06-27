use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use warp::{Filter};
mod api;
use std::env;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

const CONN: &str = "postgresql://rustuser:pgpwd4habr@postgres:5432/rustdb";

fn with_pool(
    pool: ConnectionPool,
) -> impl Filter<Extract = (ConnectionPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

#[tokio::main]
async fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let manager = PostgresConnectionManager::new_from_stringlike(CONN, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let route_index = warp::path!("index")
        .and(warp::fs::file("static/pages/index.html"));

    let route_index_empty = warp::path::end()
        .and(warp::fs::file("static/pages/index.html")); 

    let route_get_tasks = warp::path!("get-tasks")
        .and(with_pool(pool.clone()))
        .and_then(api::get_tasks);

    let route_create_task = warp::path!("add-task")
        .and(with_pool(pool))
        .and_then(api::add_task);

    let routes = route_index
        .or(route_index_empty)
        .or(route_get_tasks)
        .or(route_create_task);

    println!("Server success started: 0.0.0.0:3030");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}