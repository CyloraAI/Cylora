use actix_web::{post, web, HttpResponse, Responder};

#[post("/train/{agent_id}")]
async fn train_agent(agent_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Training agent {}", agent_id))
}

pub fn configure_training(cfg: &mut web::ServiceConfig) {
    cfg.service(train_agent);
}
