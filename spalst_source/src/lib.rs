//! SPDX-License-Identifier: GPL-3.0-only

// #![feature(inherent_associated_types)]

mod declarative_macros;
mod enums;
mod structs;
mod traits;
mod utils;

use crate::structs::{App, ArgsParser, Logger};
use clap::Parser as _;
use color_eyre::eyre::Result;
use spalst_updater::structs::ProgramUpdater;
use tracing::instrument;

/// Initializes the program.
/// Logic is handed off to `App::run()`.
#[expect(clippy::missing_errors_doc, reason = "main function")]
#[instrument]
pub async fn run() -> Result<()> {
    println!("{}", size_of::<ProgramUpdater>()); // todo: remove

    let args: ArgsParser = ArgsParser::parse();

    let logger: Logger = Logger::try_init(&args).await?;

    let mut app: App = App::new(args, logger);

    app.run().await?;

    // Ok.
    Ok(())
}
