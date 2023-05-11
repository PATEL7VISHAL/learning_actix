use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("/app")
}

#[get("/version")]
async fn version(version: web::Data<f32>) -> impl Responder {
    HttpResponse::Ok().body(version.to_string())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/app").service(root).service(version));
}
