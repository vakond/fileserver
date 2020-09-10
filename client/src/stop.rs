//! Stop.

use crate::client;
use anyhow;
use tokio::runtime::Runtime;

pub const CMD: &str = "stop";
pub const ABOUT: &str = "Shuts down the server";

/// Executes command `client stop`.
pub fn execute() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;
    rt.block_on(client::stop())?;
    Ok(())
}
