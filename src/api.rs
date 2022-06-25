extern crate postgres;
use std::{io};
use postgres::{Client, NoTls, Error};
use actix_files::NamedFile;
use actix_web::{
    dev, error, middleware::ErrorHandlerResponse, web, Error as ActixError, HttpResponse, Result, Responder
};
use serde::{Serialize};
use std::fs::File;
use std::env;
use std::io::prelude::*; 

use crate::entitys::{Task, ResponseGetTasks};

//Возвращает главную страницу
pub async fn index() -> Result<HttpResponse, ActixError> {
    let mut file = File::open("static/pages/index.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

//Возвращает задачи
pub async fn get_tasks() -> Result<impl Responder, ActixError> {

    let mut client = Client::connect(
        "postgresql://rustuser:pgpwd4habr@localhost:5432/rustdb",
        NoTls,
    ).map_err(|_| io::Error::new(io::ErrorKind::Other, "can't connect to DB"))?;

    let mut dataTasks:Vec<Task> = Vec::new();

    for row in client.query("SELECT id, name, status FROM tasks", &[]).unwrap() {
        let (task_id, task_name, task_status) = (row.get(0), row.get(1), row.get(2));

        dataTasks.push(Task{id: task_id, name: task_name, status: task_status});
    }
    
    Ok(web::Json(ResponseGetTasks {
        message: "Success".to_string(),
        tasks: dataTasks
    }))

    //Заглушка
    // let mut tasks:Vec<Task> = Vec::new();
    // tasks.push(Task{id: 1, name: "Проснуться".to_string(), status: 1});
    // tasks.push(Task{id: 2, name: "Лечь спать".to_string(), status: 1});
    //Ok(web::Json(tasks))
}

//Добавляет задачу
pub async fn add_task() -> Result<impl Responder, Error> {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut client = Client::connect(
        "postgresql://postgres@localhost:5432/rusttasks",
        NoTls,
    )?;

    client.execute(
        "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
        &[&"user1", &"mypass", &"user@test.com"],
    )?;

    Ok(HttpResponse::Ok().body("Success create!"))
}

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
