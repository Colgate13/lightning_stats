use dotenv::dotenv;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Environments {
  pub port: u16,
  pub database_url: String
}

pub fn get_environments() -> Environments {
  dotenv().ok();

  envy::from_env::<Environments>()
   .expect("Error to load environments")
}

pub fn settings_logger() {
  unsafe {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
  }
  env_logger::init();
}
