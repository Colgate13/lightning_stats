use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use crate::infra::schema::nodes;

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(table_name = nodes)]
pub struct Node {
  pub public_key: String,
  pub alias: String,
  pub capacity: BigDecimal,
  pub first_seen: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseNodes {
  #[serde(rename = "publicKey")]
  public_key: String,
  alias: String,
  capacity: i64,
  #[serde(rename = "firstSeen")]
  first_seen: i64
}

impl Node {
  fn satoshis_to_bitcoin(satoshis: i64) -> BigDecimal {
    let sats = BigDecimal::from(satoshis);
    let bitcoin_factor = BigDecimal::from(100_000_000);
    sats / bitcoin_factor
  }

  fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(timestamp, 0)
      .unwrap_or_else(Utc::now)
  }
}

impl From<ResponseNodes> for Node {
  fn from(value: ResponseNodes) -> Self {
    Node {
      public_key: value.public_key,
      alias: value.alias,
      capacity: Node::satoshis_to_bitcoin(value.capacity),
      first_seen: Node::timestamp_to_datetime(value.first_seen)
    }
  }
}
