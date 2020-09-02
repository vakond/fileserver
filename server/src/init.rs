// init

use crate::config;
use anyhow;

pub const NAME: &str = "init";
pub const ABOUT: &str = "Initializes configuration";

/// Executes command "server init".
pub fn execute() -> anyhow::Result<()> {
    config::init()?;
    Ok(())
}
