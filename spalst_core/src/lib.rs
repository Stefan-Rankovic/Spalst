//! SPDX-License-Identifier: GPL-3.0-only

#![expect(unused, reason = "i cant bother")] // temporary; todo

mod consts;
mod enums;
mod structs;
mod traits;
mod types;
mod update; // todo: uncomment this
mod utils;

use crate::{
    consts::LOGS_PATH,
    structs::{App, EnsureTerminalRestore},
    utils::set_up_logging,
};
use clap::Parser as _;
use color_eyre::eyre::Result;
use log::info;
use spalstatui::structs::Terminal;
use std::{
    env,
    path::{Path, PathBuf},
};
use structs::ArgsParser;
use tokio::fs::{self, DirEntry, ReadDir};

#[expect(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    reason = "This function is the lib.rs equivalent of main(), documenting all errors is unfeasible and unnecessary."
)]
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
        logs.sort_by_cached_key(DirEntry::file_name);
        let _: Option<DirEntry> = logs.pop();
        for log_file in logs {
            fs::remove_file(log_file.path()).await?;
        }
        info!("Cleaned up all old logs");
    }

    // Update the program
    //updater().await?; //todo: comment out when there actually is a release

    // Ensure that the terminal is always restored to how it was before the program started
    let _restore: EnsureTerminalRestore = EnsureTerminalRestore;

    // Initialize the UI
    let terminal: Terminal = spalstatui::init();

    // Initialize the App
    let mut app: App = App::try_new().await?;

    // Run the App
    app.run(terminal).await?;

    // Ok.
    Ok(())
}
