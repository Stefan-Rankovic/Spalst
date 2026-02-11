//! SPDX-License-Identifier: GPL-3.0-only

use crate::enums::LevelFilterWrapper;
use chrono::{
    Local,
    format::{DelayedFormat, StrftimeItems},
};
use color_eyre::eyre::{Result, bail};
use log::info;
use simplelog::{Config, WriteLogger};
use std::{
    fs::File, // Use std::fs::File instead of tokio::fs::File because the tokio version doesn't implement std::io::Write which WriteLogger::init() requires.
    path::{Path, PathBuf},
};
use tokio::fs::create_dir;

pub async fn set_up_logging(logs_path: &Path, log_level: LevelFilterWrapper) -> Result<()> {
    if !logs_path.try_exists()? {
        // If the logs path doesn't exist, create it.
        create_dir(logs_path).await?;
    } else if !logs_path.is_dir() {
        // If the logs path does exist, but isn't a directory, bail.
        bail!("{} exists but is not a directory.", logs_path.display());
    } else {
        // The logs path is a directory that exists. No need to do anything.
    }

    // Current time
    let current_time_formatted: DelayedFormat<StrftimeItems<'_>> =
        Local::now().format("%Y-%m-%d_%H-%M-%S");

    // Current log path.
    let current_log_file_path: PathBuf = logs_path.join(format!("{current_time_formatted}.log"));

    // If the current log file path exists, bail.
    if current_log_file_path.try_exists()? {
        bail!(
            "{} exists. It was not created by this program, and the {} directory should be left for this program and this program only to manage.",
            current_log_file_path.display(),
            logs_path.display(),
        );
    }

    // Set up logging
    WriteLogger::init(
        log_level.into(),
        Config::default(),
        File::create(current_log_file_path)?,
    )?;

    info!("Started logging at {current_time_formatted}");

    // Ok.
    Ok(())
}
