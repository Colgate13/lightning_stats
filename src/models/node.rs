use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::infra::schema::nodes;

#[derive(Queryable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(table_name = nodes)]
pub struct Node {
  pub public_key: String,
  pub alias: String,
  pub capacity: BigDecimal,
  pub first_seen: DateTime<Utc>,
}
