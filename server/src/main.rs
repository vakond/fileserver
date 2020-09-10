//! Server.

mod api;
mod config;
mod init;
mod server;
mod start;

#[macro_use]
extern crate serde_derive; // for config

use clap::{App, SubCommand};

fn main() {
    if let Err(err) = run(application()) {
        eprintln!("Error: {}", err);
    }
}

/// Dispatches commands.
fn run(mut app: App) -> anyhow::Result<()> {
    let matches = app.clone().get_matches();

    if matches.subcommand_matches(init::NAME).is_some() {
        return init::execute();
    }

    if matches.subcommand_matches(start::NAME).is_some() {
        return start::execute();
    }

    app.print_help()?;
    println!("");
    Ok(())
}

/// Creates CLI UI.
fn application() -> App<'static, 'static> {
    App::new(config::APP)
        .about("Exposes files via gRPC")
        .version("0.1")
        .subcommand(SubCommand::with_name(init::NAME).about(init::ABOUT))
        .subcommand(SubCommand::with_name(start::NAME).about(start::ABOUT))
}
