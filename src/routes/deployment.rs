use actix_web::{post, web, HttpResponse, Responder};

#[post("/deploy/{agent_id}")]
async fn deploy_agent(agent_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Deploying agent {}", agent_id))
}

pub fn configure_deployment(cfg: &mut web::ServiceConfig) {
    cfg.service(deploy_agent);
}
