use std::time::Duration;
use actix_web::{web::{self, Data}, Result};
use diesel::{Connection, RunQueryDsl};
use log::{error as logger_error};
use r2d2::Error;
use tokio::time::interval;

use crate::{infra::database::DatabaseHandler, models::node::{Node, ResponseNodes}};
use crate::infra::schema::nodes::dsl::{nodes};

pub async fn get_nodes(pool_handler: Data<DatabaseHandler>) -> Result<web::Json<Vec<Node>>> {
    let result = web::block(move || -> Result<Vec<Node>, Error> {
        let mut database_connection = pool_handler.get_connection()?;

        Ok(nodes.load::<Node>(&mut database_connection).unwrap_or_else(|error| {
          logger_error!("Error in node execute: {error}");
          vec![]
        }))
    })
    .await?;

  match result {
    Ok(data) => Ok(web::Json(data)),
    Err(_) => Ok(web::Json(vec![]))
  }
}

pub async fn sync_nodes(pool_handler: &DatabaseHandler) -> Result<(), Box<dyn std::error::Error>> {
  let mut database_connection = pool_handler.get_connection()?;
  let client = reqwest::Client::new();

  let response = client
    .get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
    .timeout(Duration::from_secs(30))
    .send()
    .await?
    .error_for_status()?;
  let nodes_data = response.json::<Vec<ResponseNodes>>().await?;

  database_connection.transaction(|connection| -> Result<(), Box<dyn std::error::Error>> {
    diesel::delete(nodes).execute(connection)?;

    let new_nodes: Vec<Node> = nodes_data
      .into_iter()
      .map(Node::from)
      .collect();

    diesel::insert_into(nodes)
      .values(&new_nodes)
      .execute(connection)?;

    Ok(())
  })
}

pub async fn sync_nodes_routine(pool_handler: DatabaseHandler) {
  let mut interval = interval(Duration::from_secs(30));

  loop {
    interval.tick().await;
    if let Err(error) = sync_nodes(&pool_handler).await {
      logger_error!("Error in sync nodes routine: {error}");
    }
  }
}
