use actix_files::NamedFile;
use postgres::{Client, Error, NoTls};
use actix_web::{
    dev, error, middleware::ErrorHandlerResponse, web, Error, HttpResponse, Result, Responder
};
use serde::{Serialize};
use std::fs::File;
use std::env;
use std::io::prelude::*;

use crate::entitys::Task;

//Возвращает главную страницу
pub async fn index() -> Result<HttpResponse, Error> {
    let mut file = File::open("static/pages/index.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

//Возвращает задачи
pub async fn get_tasks() -> Result<impl Responder> {

    let mut tasks:Vec<Task> = Vec::new();
    tasks.push(Task{id: 1, name: "Проснуться".to_string(), status: 1});
    tasks.push(Task{id: 2, name: "Лечь спать".to_string(), status: 1});

    Ok(web::Json(tasks))
}

pub async fn get_data() -> Result<impl Responder, postgres::Error> {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut client = Client::connect(
        "postgresql://postgres@localhost:5432/rusttasks",
        NoTls,
    )?;

    let mut tasks:Vec<Task> = Vec::new();

    for row in client.query("SELECT * FROM tasks", &[])? {
        tasks.push(Task{id: row.get(0).parse::<u64>(), name: row.get(1), status: row.get(2).parse::<u64>()});
    }

    Ok(web::Json(tasks))
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
