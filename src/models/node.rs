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
  // Converts satoshis to bitcoin (1 BTC = 100,000,000 satoshis)
  fn satoshis_to_bitcoin(satoshis: i64) -> BigDecimal {
    let sats = BigDecimal::from(satoshis);
    let bitcoin_factor = BigDecimal::from(100_000_000);
    sats / bitcoin_factor
  }

  // Converts a timestamp to a DateTime<Utc>
  fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(timestamp, 0)
      .unwrap_or_else(Utc::now)
  }
}

/**
 * Converts a ResponseNodes to a Node.
 */
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

#[cfg(test)]
mod tests {
  use super::*;
  use bigdecimal::BigDecimal;
  use chrono::{Utc, TimeZone};
  use std::str::FromStr;

  #[test]
  fn test_satoshis_to_bitcoin() {
    // Test conversion: 100,000,000 satoshis = 1 BTC
    let satoshis = 100_000_000;
    let expected = BigDecimal::from_str("1").unwrap();
    assert_eq!(Node::satoshis_to_bitcoin(satoshis), expected);

    // Test conversion: 0 satoshis
    let satoshis = 0;
    let expected = BigDecimal::from_str("0").unwrap();
    assert_eq!(Node::satoshis_to_bitcoin(satoshis), expected);

    // Test conversion: 50,000,000 satoshis = 0.5 BTC
    let satoshis = 50_000_000;
    let expected = BigDecimal::from_str("0.5").unwrap();
    assert_eq!(Node::satoshis_to_bitcoin(satoshis), expected);

    // Test conversion: 210,000,000,000 satoshis = 2100 BTC
    let satoshis = 210_000_000_000;
    let expected = BigDecimal::from_str("2100").unwrap();
    assert_eq!(Node::satoshis_to_bitcoin(satoshis), expected);
  }

  #[test]
  fn test_timestamp_to_datetime() {
    // Test conversion: 1697054700 (2023-10-11 20:05:00 UTC)
    let timestamp = 1697054700;
    let expected = Utc.timestamp_opt(1697054700, 0).unwrap();
    assert_eq!(Node::timestamp_to_datetime(timestamp), expected);

    // Test conversion: should 1970-01-01 00:00:00 UTC
    let timestamp = 0;
    let expected = Utc.timestamp_opt(0, 0).unwrap();
    assert_eq!(Node::timestamp_to_datetime(timestamp), expected);

    // Test conversion: should return valid datetime before epoch
    let timestamp = -86400;
    let expected = Utc.timestamp_opt(-86400, 0).unwrap();
    assert_eq!(Node::timestamp_to_datetime(timestamp), expected);
  }

  #[test]
  fn test_response_nodes_to_node_conversion() {
    let response = ResponseNodes {
      public_key: "abc123".to_string(),
      alias: "test_node".to_string(),
      capacity: 100_000_000,
      first_seen: 1522941222,
    };

    let node = Node::from(response.clone());
    assert_eq!(node.public_key, response.public_key);
    assert_eq!(node.alias, response.alias);
    assert_eq!(node.capacity, BigDecimal::from_str("1").unwrap());
    assert_eq!(node.first_seen, Utc.timestamp_opt(1522941222, 0).unwrap());

    let response_zero = ResponseNodes {
      public_key: "xyz789".to_string(),
      alias: "zero_node".to_string(),
      capacity: 0,
      first_seen: 0,
    };

    let node_zero = Node::from(response_zero.clone());
    assert_eq!(node_zero.public_key, response_zero.public_key);
    assert_eq!(node_zero.alias, response_zero.alias);
    assert_eq!(node_zero.capacity, BigDecimal::from_str("0").unwrap());
    assert_eq!(node_zero.first_seen, Utc.timestamp_opt(0, 0).unwrap());
  }
}
