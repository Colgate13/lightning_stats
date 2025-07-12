use dotenv::dotenv;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Environments {
  pub port: u16,
  pub database_url: String
}

/**
 * Retrieves the environment variables for the application.
 * Uses dotenv to load environment variables from a .env file.
 */
pub fn get_environments() -> Environments {
  dotenv().ok();

  envy::from_env::<Environments>()
   .expect("Error to load environments")
}

/**
 * Initializes the logger for the application.
 * Sets the log level to info and enables backtrace.
 */
pub fn settings_logger() {
  unsafe {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
  }
  env_logger::init();
}
