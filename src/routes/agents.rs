use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/agents")]
async fn get_agents() -> impl Responder {
    HttpResponse::Ok().body("Listing agents")
}

#[post("/agents")]
async fn create_agent() -> impl Responder {
    HttpResponse::Ok().body("Agent created")
}

pub fn configure_agents(cfg: &mut web::ServiceConfig) {
    cfg.service(get_agents);
    cfg.service(create_agent);
}
