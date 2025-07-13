use actix_web::{web::{self, Data}, Result};
use diesel::RunQueryDsl;
use log::{error as logger_error};
use r2d2::Error;

use crate::{infra::{database::DatabaseHandler}, models::node::Node};
use crate::infra::schema::nodes::dsl::{nodes};

pub async fn execute(pool_handler: Data<DatabaseHandler>) -> Result<web::Json<Vec<Node>>> {
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
