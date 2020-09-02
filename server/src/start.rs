// start

use crate::{config, server};
use tokio::runtime::Runtime;

pub const NAME: &str = "start";
pub const ABOUT: &str = "Starts operation";

// Executes command "server start".
pub fn execute() -> anyhow::Result<()> {
    let cfg = config::load()?;
    let mut rt = Runtime::new()?;
    rt.block_on(server::serve(cfg))?;
    Ok(())
}
