use std::{collections::HashMap, time::Instant};
use actix_web::{web::{self, Data}, Result};
use diesel::{sql_query, RunQueryDsl};

use crate::infra::{database::DatabaseHandler, Application, ApplicationStatus, EApplications, StatusResponder};

/**
 * Executes the status check for the application.
 */
pub async fn execute(pool_handler: Data<DatabaseHandler>) -> Result<web::Json<StatusResponder>> {
  let mut applications: HashMap<EApplications, Application> = HashMap::new();

  let start = Instant::now();
  let postgres_status = web::block(move || -> ApplicationStatus {
    let current_connection = match pool_handler.pool.get() {
      Ok(connection) => Ok(connection),
      Err(_) => Err(false)
    };

    if current_connection.is_err() {
      return ApplicationStatus::Inactive;
    }

    let mut current_connection = current_connection.unwrap();

    let select_result = sql_query("SELECT 1;").execute(&mut current_connection).is_ok();
    if !select_result {
      return ApplicationStatus::Inactive;
    }

    ApplicationStatus::Active
  }).await?;
  let duration = start.elapsed();
  applications.insert(EApplications::Postgres, Application {
    status: postgres_status,
    response_time: duration.as_millis()
  });

  let status = if applications.values().any(|app| app.status == ApplicationStatus::Inactive) {
    ApplicationStatus::Inactive
  } else {
    ApplicationStatus::Active
  };

  Ok(web::Json(StatusResponder {
      status,
      applications
  }))
}
