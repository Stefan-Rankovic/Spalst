//! SPDX-License-Identifier: GPL-3.0-only}

use crate::{enums::LogLevel, log_and_bail, structs::ArgsParser, utils::game_directory};
use chrono::Local;
use color_eyre::eyre::Result;
use std::{
    fs::{File, remove_file as sync_remove_file},
    path::PathBuf,
};
use tokio::fs::{self, DirEntry, ReadDir, create_dir};
use tracing::{debug, error, info, instrument, level_filters::LevelFilter, trace};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

/// A logger! Well, actually not. A wrapper over one.
///
/// Has many useful methods to interact with `tracing`. Such as automatically intializing, just
/// with some arguments.
///
/// If the arguments include `clean_log_after` set to `true`, the instance will automatically, when
/// dropped, delete the log file it created.
#[derive(Debug)]
pub struct Logger {
    /// The max level used for logging.
    max_level: LogLevel,

    /// At what time logging has started.
    /// Initially set to when `try_init()` was called but recomputed for when logging actually started.
    start_time: String,

    /// Whether to delete the current log file after the program finishes.
    clean_current_file_after_use: bool,
    /// The path to the current log file.
    current_file_path: PathBuf,
}

impl Drop for Logger {
    fn drop(&mut self) {
        trace!("Dropping Logger...");
        if !self.clean_current_file_after_use {
            trace!("clean_current_file_after_use was not true. Exiting...");
            return;
        }
        debug!("While dropping logger, clean_current_file_after_use was true. Deleting file...");
        if let Err(error) = sync_remove_file(&self.current_file_path) {
            error!("Failed to remove current log file with error \"{error}\".");
        }
    }
}

impl Logger {
    /// The current time, but formatted.
    fn current_time() -> String {
        format!("{}", Local::now().format("%Y-%m-%d_%H-%M-%S"))
    }

    /// Initializes the struct.
    ///
    /// Run this, set it to a variable with an underscore at the start of its name, and forget it.
    ///
    /// Note that if the passed arguments include `clean_log_after` set to `true`, you may want to
    /// avoid dropping the instance early.
    #[instrument]
    pub async fn try_init(args: &ArgsParser) -> Result<Self> {
        let current_time: String = Self::current_time();
        let logs_path: PathBuf = Self::logs_path();

        let instance: Self = Self {
            max_level: args.log_level,
            clean_current_file_after_use: args.clean_log_after,
            current_file_path: logs_path.join(format!("{current_time}_local_time.log")), // Should be before_start_time because start_time takes current_time
            start_time: current_time,
        };

        instance.start_logging().await?;

        if args.clean_previous_logs {
            instance.clean_previous_logs().await?;
        }

        // Ok.
        Ok(instance)
    }

    /// Deletes all logs, except the last one.
    #[instrument(skip(self))]
    pub async fn clean_previous_logs(&self) -> Result<()> {
        let logs_path: PathBuf = Self::logs_path();

        let mut entries: Vec<DirEntry> = Vec::new();
        let mut read_dir: ReadDir = fs::read_dir(logs_path).await?;
        while let Some(dir_entry) = read_dir.next_entry().await? {
            entries.push(dir_entry);
        }

        entries.sort_unstable_by_key(DirEntry::file_name);

        // split_last in order to not delete the last element (the current log file)
        if let Some((_, to_delete)) = entries.split_last() {
            for entry in to_delete {
                fs::remove_file(entry.path()).await?;
                debug!("Removed previous log {}", entry.path().display());
            }
        }

        info!("Removed all previous logs.");

        // Ok.
        Ok(())
    }

    /// Create `self.logs_path()` directory if it doesn't already exists.
    ///
    /// # Errors
    /// If `self.logs_path()` already exists but is not a directory.
    async fn ensure_dir(&self) -> Result<()> {
        let logs_path = Self::logs_path();
        if !logs_path.try_exists()? {
            // If the logs path doesn't exist, create it.
            create_dir(&logs_path).await?;
            info!("Created {}", logs_path.display());
        } else if !logs_path.is_dir() {
            // If the logs path does exist, but isn't a directory, bail.
            log_and_bail!("{} exists but is not a directory.", logs_path.display());
        } else {
            // The logs path is a directory that exists. No need to do anything.
        }
        // Ok.
        Ok(())
    }

    #[instrument(skip(self))]
    /// Start logging.
    pub async fn start_logging(&self) -> Result<()> {
        self.ensure_dir().await?;

        // If the current log file path already exists, bail.
        if self.current_file_path.try_exists()? {
            log_and_bail!("{} exists", self.current_file_path.display());
        }

        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::from(self.max_level))
            .with_writer(File::create(&self.current_file_path)?)
            .finish()
            .with(ErrorLayer::default())
            .init();

        info!("Started logging at {}", self.start_time);
        info!("Log path is {}", self.current_file_path.display());
        info!(
            "Set up to{} delete the current log after the program finishes.",
            if self.clean_current_file_after_use {
                ""
            } else {
                " not"
            }
        );

        // Ok.
        Ok(())
    }

    /// The parent directory of all logs.
    pub fn logs_path() -> PathBuf {
        game_directory().join("logs/")
    }
}
