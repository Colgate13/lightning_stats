use actix_web::{web, Result};
use serde::{Serialize};

#[derive(Serialize)]
pub enum Status {
  Active
}

#[derive(Serialize)]
pub struct StatusResponder {
  status: Status
}

pub async fn execute() -> Result<web::Json<StatusResponder>> {
  Ok(web::Json(StatusResponder {
      status: Status::Active
  }))
}
