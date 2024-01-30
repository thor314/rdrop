#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use error::MyError;
use log::{error, info};

mod cli;
mod error;
#[cfg(test)] mod tests;
mod utils;

fn main() -> Result<(), MyError> {
  let _cli = utils::setup()?;
  info!("hello thor");

  Ok(())
}