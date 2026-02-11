//! SPDX-License-Identifier: GPL-3.0-only

pub mod consts;
pub mod enums;
pub mod structs;
pub mod traits;
pub mod types;
pub mod update;
pub mod utils;

#[macro_use]
extern crate log;

use crate::{
    consts::LOGS_PATH,
    structs::{App, EnsureTerminalRestore},
    update::updater,
    utils::set_up_logging,
};
use clap::Parser;
use color_eyre::eyre::Result;
use log::info;
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::{
    env,
    io::Stdout,
    path::{Path, PathBuf},
};
use structs::ArgsParser;
use tokio::fs::{self, DirEntry, ReadDir};

pub async fn run() -> Result<()> {
    // Get the arguments
    let args: ArgsParser = ArgsParser::parse();

    // Define some paths
    let executable_path: PathBuf = env::current_exe()?;
    let game_path: &Path = executable_path.parent().unwrap();
    let logs_path: &Path = &game_path.join(LOGS_PATH);

    // Set up logging
    set_up_logging(logs_path, args.log_level).await?;

    // Clean up old args if the arguments say so
    if args.clean_logs {
        let mut read_dir: ReadDir = fs::read_dir(logs_path).await?;
        let mut logs: Vec<DirEntry> = {
            let mut logs: Vec<DirEntry> = Vec::new();
            while let Some(dir_entry) = read_dir.next_entry().await? {
                logs.push(dir_entry);
            }
            logs
        };
        // Preserve the latest entry (current entry).
        logs.sort_by_cached_key(|log_file| log_file.file_name());
        logs.pop();
        for log_file in logs {
            fs::remove_file(log_file.path()).await?;
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
