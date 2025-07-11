use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, Eq, PartialEq, Hash)]
pub enum EApplications {
  Postgres
}

#[derive(Serialize, PartialEq)]
pub enum ApplicationStatus {
  Active,
  Inactive
}

#[derive(Serialize)]
pub struct Application {
  pub status: ApplicationStatus,
  pub response_time: u128
}

#[derive(Serialize)]
pub struct StatusResponder {
  pub status: ApplicationStatus,
  pub applications: HashMap<EApplications, Application>
}

pub mod environment;
pub mod database;
