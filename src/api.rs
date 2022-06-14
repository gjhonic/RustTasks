use actix_files::NamedFile;
use actix_web::{
    dev, error, middleware::ErrorHandlerResponse, web, Error, HttpResponse, Result, Responder
};
use serde::{Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize)]
pub struct Task {
    id: u64,
    name: String
}

pub async fn index() -> Result<HttpResponse, Error> {
    let mut file = File::open("templates/index.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

pub async fn get_tasks() -> Result<impl Responder> {
    let mut tasks:Vec<Task> = Vec::new();
    tasks.push(Task{id: 1, name: "Проснуться".to_string()});
    tasks.push(Task{id: 2, name: "Лечь спать".to_string()});
    Ok(web::Json(tasks))
}

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
