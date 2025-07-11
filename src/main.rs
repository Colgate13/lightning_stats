use actix_web::{web, App, HttpServer};

mod infra;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infra::environment::settings_logger();
    infra::database::migrations_up();

    let pool_handler= infra::database::PoolHandler::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool_handler.clone()))
            .route("/", web::get().to(services::status::execute))
            .route("/status", web::get().to(services::status::execute))
    })
    .bind(("0.0.0.0", infra::environment::get_environments().port))?
    .run()
    .await
}
