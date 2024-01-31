#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use std::{env, fs, os::unix::fs::PermissionsExt, path::PathBuf};

use error::MyError;
use log::{error, info, trace};
use regex::Regex;

mod cli;
mod error;
#[cfg(test)] mod tests;
mod utils;

lazy_static::lazy_static! {
    static ref USER: String = env::var("USER").expect("USER environment variable not found");
    static ref DISPLAY: String = env::var("DISPLAY").expect("DISPLAY environment variable not found");
    /// tmp dir for process ids etc.
    static ref MUTDROP_PATH: PathBuf = PathBuf::from(format!("/tmp/tdrop_{}_{}", *USER, *DISPLAY));
    static ref LOG_FILE: PathBuf= env::var("RDROP_LOG_FILE").unwrap_or(format!("{}/{}", MUTDROP_PATH.to_str().unwrap(), "log")).into();
    static ref NOAUTOHIDE_FILE: PathBuf = MUTDROP_PATH.join("/no_autohide");
    static ref GEO_DIR: PathBuf = MUTDROP_PATH.join("/geometries");
    static ref WID_DIR: PathBuf = MUTDROP_PATH.join("/wids");
    static ref CLASS_DIR: PathBuf = MUTDROP_PATH.join("/classes");
    static ref HIDE_DIR: PathBuf = MUTDROP_PATH.join("/auto_hidden");
    /// Regular expression for floating window managers
    static ref FLOATING_WMS_REGEXP: Regex = Regex::new(
        r"Openbox|pekwm|Fluxbox|Blackbox|xfwm4|Metacity|FVWM|Sawfish|GoomwW|Mutter|GNOME Shell|Mutter \(Muffin\)|KWin|Metacity \(Marco\)|[Cc]ompiz"
    ).unwrap();
    /// Regular expression for geometry configuration
    static ref GEO_REGEXP: Regex = Regex::new(
        r"^xoff=-?[0-9]+\nyoff=-?[0-9]+\nwidth=?[0-9]+\nheight=?[0-9]+$"
    ).unwrap();
}

fn main() -> Result<(), MyError> {
  let _cli = utils::setup()?;
  create_static_directories()?;

  // Regular Expressions
  let floating_wms_regexp = Regex::new(
    r"Openbox|pekwm|Fluxbox|Blackbox|xfwm4|Metacity|FVWM|Sawfish|GoomwW|Mutter|GNOME Shell|Mutter \(Muffin\)|KWin|Metacity \(Marco\)|[Cc]ompiz",
  )?;
  let geo_regexp = Regex::new(r"^xoff=-?[0-9]+\nyoff=-?[0-9]+\nwidth=?[0-9]+\nheight=?[0-9]+$")?;

  Ok(())
}

/// Creating the tmp directories and set their permissions
fn create_static_directories() -> Result<(), MyError> {
  for d in [&*HIDE_DIR, &*CLASS_DIR, &*GEO_DIR, &*WID_DIR] {
    fs::create_dir_all(d)?;
    trace!("Created directory: {}", d.to_str().unwrap());
    let perms = std::fs::Permissions::from_mode(0o700);
    trace!("with perms: {:?}", perms);
    fs::set_permissions(d, perms)?;
  }
  Ok(())
}
