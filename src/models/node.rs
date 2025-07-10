use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Node {
  pub public_key: String,
  pub alias: String,
  pub capacity: BigDecimal,
  pub first_seen: DateTime<Utc>,
}
