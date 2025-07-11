use diesel::{r2d2::{ConnectionManager, Pool, PooledConnection}, Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use super::environment::{get_environments};
pub type PgPoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PoolHandler {
  pub pool: Pool<ConnectionManager<PgConnection>>
}

impl PoolHandler {
  pub fn new() -> Self {
    Self { pool: establish_pool_manager() }
  }

  pub fn get_connection(&self) -> PgPoolConnection {
    self.pool.get().unwrap()
  }
}

pub fn establish_connection() -> PgConnection {
  let connection = PgConnection::establish(&get_environments().database_url);

  connection.unwrap_or_else(|_| panic!("Error connection to database"))
}

pub fn establish_manager() -> ConnectionManager<PgConnection> {
  ConnectionManager::<PgConnection>::new(&get_environments().database_url)
}

pub fn establish_pool_manager() -> Pool<ConnectionManager<PgConnection>> {
  let connection_manager = establish_manager();

  let pool = match Pool::builder().build(connection_manager) {
    Err(error) => {
      panic!("Failed to establish pool: {}", error.to_string())
    }
    Ok(pool) => pool
  };

  pool
}

pub fn migrations_up() -> bool {
  const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
  let mut connection = establish_connection();

  match connection
    .run_pending_migrations(MIGRATIONS)
    {
      Err(error) => {
        panic!("Failed to run migrations: {}", error.to_string())
      },
      _ => true
    }
}
