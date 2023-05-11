#![allow(unused)]

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[post("/greet")]
async fn greet(name: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}", name))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There !")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(greet)
            .route("/manual", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
