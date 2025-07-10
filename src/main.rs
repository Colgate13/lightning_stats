use actix_web::{web, App, HttpServer};

mod infra;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infra::environment::settings_logger();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(services::status::execute))
            .route("/status", web::get().to(services::status::execute))
    })
    .bind(("0.0.0.0", infra::environment::get_environments().port))?
    .run()
    .await
}
