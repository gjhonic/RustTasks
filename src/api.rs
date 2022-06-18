extern crate postgres;

use actix_files::NamedFile;
use postgres::{Client, Error, NoTls};
use actix_web::{
    dev, error, middleware::ErrorHandlerResponse, web, Error as ActixError, HttpResponse, Result, Responder
};
use serde::{Serialize};
use std::fs::File;
use std::env;
use std::io::prelude::*;

const CONN : &str = "postgresql://postgres@localhost:5432/rusttasks"; //&env::var("DATABASE_URL").expect("DATABASE_URL must be set");

use crate::entitys::{Task, ResponseTasksJson};

//Возвращает главную страницу
pub async fn index() -> Result<HttpResponse, ActixError> {
    let mut file = File::open("static/pages/index.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

//Возвращает задачи (Заглушка)
pub async fn get_tasks() -> Result<impl Responder> {

    let mut tasks:Vec<Task> = Vec::new();
    tasks.push(Task{id: 1, name: "Проснуться".to_string(), status: 1});
    tasks.push(Task{id: 2, name: "Лечь спать".to_string(), status: 1});

    Ok(web::Json(tasks))
}

//Возвращает список задач
pub async fn get_data() -> Result<impl Responder, ActixError> {

    let client = Client::connect(
        CONN,
        NoTls,
    );

    let mut tasks:Vec<Task> = Vec::new();

    match client {
        Ok(mut good_client) => 
            for row in good_client.query("SELECT id, name, status FROM tasks", &[]).unwrap() {
                let (task_id, task_name, task_status) = (row.get(0), row.get(1), row.get(2));

                tasks.push(Task{id: task_id, name: task_name, status: task_status});
            },
        Err(e) => {
            return Ok(web::Json("{'status': 'error'}"));
        }, 
    };

    let responseTasksJson = ResponseTasksJson {
        status: "Ok".to_string(),
        message: "Success".to_string(),
        tasks: tasks
    };

    Ok(web::Json(responseTasksJson))
}

//Добавляет задачу
// pub async fn add_task() -> Result<impl Responder, Error> {

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

//     let mut client = Client::connect(
//         "postgresql://postgres@localhost:5432/rusttasks",
//         NoTls,
//     )?;

//     let mut tasks:Vec<Task> = Vec::new();

//     for row in client.query("SELECT * FROM tasks", &[])? {
//         tasks.push(Task{id: row.get(0).parse::<u64>(), name: row.get(1), status: row.get(2).parse::<u64>()});
//     }

//     Ok(web::Json(tasks))
// }



//Методы ошибок
pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .into_response(res.request())
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .into_response(res.request())
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res.into_response(new_resp)))
}
