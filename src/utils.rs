use anyhow::{anyhow, Result};
use clap::Parser;
use log::{info, trace};

use crate::{cli::Cli, error::MyError};

/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<Cli, MyError> {
  dotenv::dotenv().ok();

  let env = env_logger::Env::new().filter("MY_LOG").write_style("MY_LOG_STYLE");
  env_logger::init();
  if std::env::var("DOTENV_OK").is_ok() {
    info!("loaded dotenv");
    trace!("loaded dotenv");
  } else {
    return Err(anyhow!("failed to load dotenv").into());
  }

  let args = Cli::parse();
  Ok(args)
}
