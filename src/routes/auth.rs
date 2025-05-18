use actix_web::{post, HttpResponse, Responder};

#[post("/auth/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Logged in")
}

pub fn configure_auth(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(login);
}
