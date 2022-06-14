use std::{io};

use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{
    http,
    middleware::{ErrorHandlers, Logger},
    web, App, HttpServer,
};
use dotenv::dotenv;

//use std::error::Error;

mod api;

static SESSION_SIGNING_KEY: &[u8] = &[0; 32];

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://0.0.0.0:3030");

    HttpServer::new(move || {
        log::debug!("Constructing the App");

        let session_store = CookieSession::signed(SESSION_SIGNING_KEY).secure(false);

        let error_handlers = ErrorHandlers::new()
            .handler(
                 http::StatusCode::INTERNAL_SERVER_ERROR,
                 api::internal_server_error,
             )
            .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
            .handler(http::StatusCode::NOT_FOUND, api::not_found);

        App::new()
            .wrap(Logger::default())
            .wrap(session_store)
            .wrap(error_handlers)
            .service(web::resource("/get-tasks").route(web::get().to(api::get_tasks)))
            .service(web::resource("/index").route(web::get().to(api::index)))
            .service(web::resource("/").route(web::get().to(api::index)))
            .service(Files::new("/static", "./static"))
    })
    .bind(("0.0.0.0", 3030))?
    .workers(2)
    .run()
    .await
}
