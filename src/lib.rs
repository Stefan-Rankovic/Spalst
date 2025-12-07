//! SPDX-License-Identifier: GPL-3.0-only

pub mod consts;
pub mod enums;
pub mod structs;
pub mod traits;
pub mod update;
pub mod utils;

#[macro_use]
extern crate log;

use crate::{
    consts::LOGS_PATH,
    enums::LevelFilterWrapper,
    structs::{App, EnsureTerminalRestore},
    update::updater,
    utils::set_up_logging,
};
use chrono::{
    Local, Utc,
    format::{DelayedFormat, StrftimeItems},
};
use clap::Parser;
use color_eyre::eyre::{Result, bail};
use log::{LevelFilter, info};
use ratatui::{Terminal, prelude::CrosstermBackend};
use simplelog::{Config, WriteLogger};
use std::{
    env,
    fs::{self, File, create_dir},
    io::{self, Stdout},
    path::{Path, PathBuf},
};
use structs::ArgsParser;

pub async fn run() -> Result<()> {
    // Get the arguments
    let args: ArgsParser = ArgsParser::parse();
    // Define some paths
    let executable_path: PathBuf = env::current_exe()?;
    let game_path: &Path = executable_path.parent().unwrap();
    let logs_path: &Path = &game_path.join(LOGS_PATH);
    // Set up logging
    set_up_logging(logs_path, args.log_level)?;
    // Clean up old args if the arguments say so
    if args.clean_logs {
        let logs_with_errors: Vec<Result<fs::DirEntry, io::Error>> =
            fs::read_dir(logs_path)?.collect();
        let mut logs: Vec<fs::DirEntry> = Vec::new();
        for log_file_error in logs_with_errors {
            logs.push(log_file_error?);
        }
        logs.sort_by_cached_key(|log_file| log_file.file_name());
        logs.pop();
        for log_file in logs {
            fs::remove_file(log_file.path())?;
        }
        info!("Cleaned up all old logs");
    };
    // Update the program
    //updater().await?; //todo: comment out when there actually is a release
    // Ensure that the terminal is always restored to how it was before the program started
    let _restore: EnsureTerminalRestore = EnsureTerminalRestore;
    // Initialize the UI
    let terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();
    // Initialize the App
    let mut app: App = App::try_new().await?;
    // Run the App
    app.run(terminal).await?;
    // Ok.
    Ok(())
}
