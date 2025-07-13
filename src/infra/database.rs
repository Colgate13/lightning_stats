use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection}, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::Error;
use log::{error as logger_error};

use super::environment::{get_environments};

#[derive(Clone)]
pub struct DatabaseHandler {
  pub pool: Pool<ConnectionManager<PgConnection>>
}

/**
 * Establishes a connection to the database and handler connection.
 */
impl DatabaseHandler {
  pub fn new() -> Self {
    Self { pool: DatabaseHandler::establish_pool_manager() }
  }

  pub fn establish_connection() -> Result<PgConnection, diesel::ConnectionError> {
    PgConnection::establish(&get_environments().database_url)
  }

  pub fn establish_manager() -> ConnectionManager<PgConnection> {
    ConnectionManager::<PgConnection>::new(&get_environments().database_url)
  }

  pub fn establish_pool_manager() -> Pool<ConnectionManager<PgConnection>> {
    let connection_manager = DatabaseHandler::establish_manager();

    match Pool::builder().build(connection_manager) {
      Err(error) => {
        panic!("Failed to establish pool: {error}")
      }
      Ok(pool) => pool
    }
  }

  pub fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
    self
      .pool
      .get()
      .map_err(|err| {
        logger_error!("Error to get database connection: {err}");
        err
      })
  }

  /**
   * Runs the migrations for the database.
   */
  pub fn migrations_up() -> bool {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    let connection = DatabaseHandler::establish_connection();

    if connection.is_err() {
      logger_error!("Failed to run migrations. error to get database connection");
      return false;
    }

    match connection
      .unwrap()
      .run_pending_migrations(MIGRATIONS)
      {
        Err(error) => {
          logger_error!("Failed to run migrations: {error}");
          false
        },
        _ => true
      }
  }
}
