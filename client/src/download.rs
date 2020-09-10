//! Download.

use crate::client;
use anyhow;
use tokio::runtime::Runtime;

pub const CMD: &str = "download";
pub const ARG: &str = "NAME";
pub const ABOUT: &str = "Requests a file for download";

/// Executes command `client download`.
pub fn execute(filename: &str) -> anyhow::Result<()> {
    let mut rt = Runtime::new()?;
    rt.block_on(client::download(filename))?;
    Ok(())
}
