#![allow(unused)]

pub mod api;
pub mod app;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, guard};
use std::sync::Mutex;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("/")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(app::config)
            .service(web::scope("/api").configure(api::config))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
