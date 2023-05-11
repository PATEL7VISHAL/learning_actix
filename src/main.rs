#![allow(unused)]

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{fmt::format, sync::Mutex};

async fn index(state: web::Data<AppState>, name_state: web::Data<&'static str>) -> impl Responder {
    let count = {
        let mut t = state.req_count.lock().unwrap();
        *t = *t + 1;
        *t - 1
    };
    HttpResponse::Ok().body(format!(
        "Total Request : {} name: {}",
        count,
        name_state.as_ref()
    ))

    // HttpResponse::Ok().body("Hello")
}

async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home page")
}

#[get("/sub")]
async fn sub(name_state: web::Data<&'static str>, state: web::Data<AppState>) -> impl Responder {
    let count = {
        let mut t = state.req_count.lock().unwrap();
        *t = *t + 1;
        *t - 1
    };
    HttpResponse::Ok().body(format!(
        "Total Request : {} name: {}",
        count,
        name_state.as_ref()
    ))
}

#[derive(Debug)]
struct AppState {
    req_count: std::sync::Mutex<u32>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut state = web::Data::new(AppState {
        req_count: Mutex::new(0),
    });

    let name_state = web::Data::new("Vishal");

    HttpServer::new(move || {
        App::new()
            .app_data(name_state.clone())
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index))
                    .route("/home", web::get().to(home)),
            )
            .app_data(state.clone())
            .service(web::scope("/another").service(sub))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
