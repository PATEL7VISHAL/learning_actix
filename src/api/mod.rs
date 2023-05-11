use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("/api")
}

async fn root2() -> impl Responder {
    HttpResponse::Ok().body("/api")
}

#[get("/version")]
async fn version(version: web::Data<f32>) -> impl Responder {
    HttpResponse::Ok().body(version.to_string())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(root);

    // cfg.service(web::resource("/api").route(web::get().to(root2)));

    let api_version = web::Data::new(1.3f32);
    cfg.service(web::resource("/api").app_data(api_version.clone()))
        .service(version);
}
