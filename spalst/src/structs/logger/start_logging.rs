//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{bail_log, info, structs::Logger};
use color_eyre::eyre::{OptionExt as _, Result};
use std::fs::File;
use tracing::{instrument, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

impl Logger {
    #[instrument(skip(self))]
    /// Start logging.
    pub fn start_logging(&mut self) -> Result<()> {
        Self::ensure_logs_dir()?;

        // If the current logfile path already exists, bail.
        if self.log_file.try_exists()? {
            bail_log!("{} exists", self.log_file.display());
        }

        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::from(self.max_level))
            .with_writer(File::create(&self.log_file)?)
            .finish()
            .with(ErrorLayer::default())
            .init();

        self.update_start_time();

        info!(
            "Started logging at {}",
            self.start_time
                .as_ref()
                .ok_or_eyre("Unreachable. This was set on the line above.")?
        );
        info!("Log path is {}", self.log_file.display());
        info!(
            "Set up to{} delete the current log after the program finishes.",
            if self.rm_on_drop {
                ""
            } else {
                " not"
            }
        );

        Ok(())
    }
}
