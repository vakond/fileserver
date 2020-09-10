//! Files.

use crate::client;
use anyhow;
use tokio::runtime::Runtime;

pub const CMD: &str = "files";
pub const ABOUT: &str = "Shows list of available files";

/// Executes command `client files`.
pub fn execute() -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;
    rt.block_on(client::list())?;
    Ok(())
}
