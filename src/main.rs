use actix_web::{web, App, Result, HttpServer};
use serde::{Serialize};
mod infra;

#[derive(Serialize)]
enum Status {
    Active
}

#[derive(Serialize)]
struct StatusResponder {
    status: Status
}

async fn status() -> Result<web::Json<StatusResponder>> {
    Ok(web::Json(StatusResponder {
        status: Status::Active
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infra::environment::settings_logger();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
            .route("/status", web::get().to(status))
    })
    .bind(("0.0.0.0", infra::environment::get_environments().port))?
    .run()
    .await
}
