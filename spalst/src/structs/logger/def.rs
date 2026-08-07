//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{
    statics::LOG_FILES_DIR,
    structs::{AbsolutePathBuf, ArgsParser},
};
use chrono::Utc;
use color_eyre::eyre::Result;
use tracing::{Level, instrument};

/// A wrapper over a logger.
///
/// If the arguments include `rm_log` set to `true`, the instance will automatically, when
/// dropped, delete the logfile it created.
#[derive(Debug)]
pub struct Logger {
    /// The max level used for logging.
    pub(super) max_level: Level,

    /// At what time (UTC) logging has started.
    /// Only set when logging actually started.
    pub(super) start_time: Option<String>,

    /// Whether to delete the current logfile after the program finishes.
    ///
    /// In order to actually delete the file, `successful_exit` must also be `true`.
    pub(super) rm_on_drop: bool,
    /// Whether the program exited successfully.
    pub successful_exit: bool,

    /// The path to the current logfile.
    pub(super) log_file: AbsolutePathBuf,
}

impl Logger {
    /// Makes a new `Logger` instance.
    ///
    /// `args.log` should be `true`.
    ///
    /// Avoid dropping early if `args.rm_log` is `true`.
    pub fn new(args: &ArgsParser) -> Self {
        assert!(
            args.log,
            "The arguments say logging is disabled, yet this method was called."
        );

        Self {
            max_level: args.log_level.into(),
            rm_on_drop: args.rm_log,
            successful_exit: false,
            log_file: LOG_FILES_DIR.join(format!("{}_utc.log", Self::formatted_utc_time())),
            start_time: None,
        }
    }

    /// Creates and initializes the struct.
    ///
    /// `args.log` should be `true`.
    ///
    /// Avoid dropping early if `args.rm_log` is `true`.
    #[instrument]
    pub async fn try_init_new(args: &ArgsParser) -> Result<Self> {
        let mut instance: Self = Self::new(args);

        instance.try_init(args).await.map(|()| instance)
    }

    /// Initializes the struct.
    pub async fn try_init(
        &mut self,
        args: &ArgsParser,
    ) -> Result<()> {
        self.start_logging()?;

        if args.rm_old_logs {
            self.rm_old_logs().await?;
        }

        Ok(())
    }

    /// The formatted current UTC time.
    fn formatted_utc_time() -> String {
        Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string()
    }

    /// Sets `self.start_time` to the current UTC time.
    pub(super) fn update_start_time(&mut self) {
        self.start_time = Some(Self::formatted_utc_time());
    }
}
