use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection}, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use super::environment::{get_environments};
pub type PgPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PoolHandler {
  pub pool: Pool<ConnectionManager<PgConnection>>
}

/**
 * Establishes a connection to the database and handler connection.
 */
impl PoolHandler {
  pub fn new() -> Self {
    Self { pool: PoolHandler::establish_pool_manager() }
  }

  pub fn establish_connection() -> PgConnection {
    let connection = PgConnection::establish(&get_environments().database_url);

    connection.unwrap_or_else(|_| panic!("Error connection to database"))
  }

  pub fn establish_manager() -> ConnectionManager<PgConnection> {
    ConnectionManager::<PgConnection>::new(&get_environments().database_url)
  }

  pub fn establish_pool_manager() -> Pool<ConnectionManager<PgConnection>> {
    let connection_manager = PoolHandler::establish_manager();

    match Pool::builder().build(connection_manager) {
      Err(error) => {
        panic!("Failed to establish pool: {error}")
      }
      Ok(pool) => pool
    }
  }

  #[allow(dead_code)]
  pub fn get_connection(&self) -> PgPoolConnection {
    self.pool.get().unwrap()
  }
}

/**
 * Runs the migrations for the database.
 * Returns true if the migrations were successful.
 * Panics if the migrations fail.
 */
pub fn migrations_up() -> bool {
  const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
  let mut connection = PoolHandler::establish_connection();

  match connection
    .run_pending_migrations(MIGRATIONS)
    {
      Err(error) => {
        panic!("Failed to run migrations: {error}")
      },
      _ => true
    }
}
