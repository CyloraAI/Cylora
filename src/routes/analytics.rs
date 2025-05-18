use actix_web::{get, web, HttpResponse, Responder};

#[get("/analytics/{agent_id}")]
async fn analytics(agent_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Analytics for agent {}", agent_id))
}

pub fn configure_analytics(cfg: &mut web::ServiceConfig) {
    cfg.service(analytics);
}
