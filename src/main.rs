use actix_web::{web, App, HttpServer};

mod infra;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infra::environment::settings_logger(); // Initialize logger settings
    infra::database::DatabaseHandler::migrations_up(); // Run database migrations

    let pool_handler= infra::database::DatabaseHandler::new(); // Create a new database connection pool handler
    let sync_pool_handler = pool_handler.clone(); // Clone the pool handler for the sync task

    tokio::spawn(async move { // Spawn a background task to periodically sync nodes
        services::node::sync_nodes_routine(sync_pool_handler.clone()).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool_handler.clone()))
            .route("/", web::get().to(services::status::execute))
            .route("/status", web::get().to(services::status::execute))
            .route("/nodes", web::get().to(services::node::get_nodes))
    })
    .bind(("0.0.0.0", infra::environment::get_environments().port))?
    .run()
    .await
}
