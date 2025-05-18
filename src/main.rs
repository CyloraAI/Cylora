use actix_web::{web, App, HttpServer};
use cylora_backend::routes::{agents::*, training::*, deployment::*, analytics::*, auth::*};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(configure_auth)
            .configure(configure_agents)
            .configure(configure_training)
            .configure(configure_deployment)
            .configure(configure_analytics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
