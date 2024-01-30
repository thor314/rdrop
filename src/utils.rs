use anyhow::{anyhow, Result};
use clap::Parser;
use log::trace;

use crate::{cli::Cli, error::MyError};

/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<Cli, MyError> {
  dotenv::dotenv().ok();
  // init_tracing();
  env_logger::init();
  if std::env::var("DOTENV_OK").is_ok() {
    trace!("loaded dotenv");
  } else {
    return Err(anyhow!("failed to load dotenv").into());
  }

  let args = Cli::parse();
  Ok(args)
}