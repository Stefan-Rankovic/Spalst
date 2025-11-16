pub mod consts;
pub mod enums;
pub mod renew_account;
pub mod structs;
pub mod traits;
pub mod update;
pub mod utils;

#[macro_use]
extern crate log;

use crate::{
    enums::LevelFilterWrapper,
    structs::{App, EnsureTerminalRestore},
    update::updater,
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
    io::Stdout,
    path::{Path, PathBuf},
};
use structs::ArgsParser;

pub fn set_up_logging(logs_path: &Path, log_level: LevelFilterWrapper) -> Result<()> {
    // If the logs path doesn't exist, create it. If it does and isn't a directory, return an
    // error.
    if !logs_path.try_exists()? {
        create_dir(logs_path)?;
    } else if !logs_path.is_dir() {
        bail!("{} exists but is not a directory.", logs_path.display());
    };
    // Define the current time and log file path
    let current_time_formatted: DelayedFormat<StrftimeItems> =
        Local::now().format("%Y-%m-%d_%H-%M-%S");
    let current_log_file_path: PathBuf = logs_path.join(format!("{}.log", current_time_formatted));
    // If the current log file path exists, return an error
    if current_log_file_path.try_exists()? {
        bail!(
            "{} exists. It was not created by this program, and the {} directory should be left for this program and this program only to manage.",
            current_log_file_path.display(),
            logs_path.display(),
        );
    };
    // Set up logging
    WriteLogger::init(
        log_level.into(),
        Config::default(),
        File::create(current_log_file_path)?,
    )?;
    info!("Started logging at {}", current_time_formatted);
    // Ok.
    Ok(())
}

pub async fn run() -> Result<()> {
    // Get the arguments
    let args: ArgsParser = ArgsParser::parse();
    // Define some paths
    let executable_path: PathBuf = env::current_exe()?;
    let game_path: &Path = executable_path.parent().unwrap();
    let logs_path: &Path = &game_path.join("logs/");
    // Set up logging
    set_up_logging(logs_path, args.log_level)?;
    // Clean up old args if the arguments say so
    if args.clean_logs {
        let logs_with_errors: Vec<Result<fs::DirEntry, std::io::Error>> =
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
    //updater().await?; //todo: comment out when there actuall is a release
    // Ensure that the terminal is always restored to how it was before the program started
    let _restore: EnsureTerminalRestore = EnsureTerminalRestore;
    // Initialize the UI
    let terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();
    // Initialize the App
    let mut app: App = App::try_new()?;
    // Run the App
    app.run(terminal).await?;
    // Ok.
    Ok(())
}
