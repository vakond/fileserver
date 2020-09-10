//! Client.

mod api;
mod client;
mod download;
mod files;

use anyhow;
use clap::{App, Arg, SubCommand};

/// Main function.
fn main() {
    if let Err(err) = run(application()) {
        eprintln!("Error: {}", err);
    }
}

/// Dispatches commands.
fn run(mut app: App) -> anyhow::Result<()> {
    let matches = app.clone().get_matches();

    if matches.subcommand_matches(files::CMD).is_some() {
        return files::execute();
    }

    if let Some(matches) = matches.subcommand_matches(download::CMD) {
        return download::execute(matches.value_of(download::ARG).unwrap());
    }

    app.print_help()?;
    println!();
    Ok(())
}

/// Creates CLI UI.
fn application() -> App<'static, 'static> {
    App::new("client")
        .about("Sends requests to the fileserver")
        .version("0.1")
        .subcommand(SubCommand::with_name(files::CMD).about(files::ABOUT))
        .subcommand(
            SubCommand::with_name(download::CMD)
                .about(download::ABOUT)
                .arg(
                    Arg::with_name(download::ARG)
                        .help("Filename to download")
                        .required(true)
                        .index(1),
                ),
        )
}
