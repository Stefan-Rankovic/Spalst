//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{bail_log, statics::LOG_FILES_DIR, structs::Logger};
use color_eyre::eyre::Result;
use std::fs::create_dir_all;
use tracing::info;

impl Logger {
    /// Create `LOGS_FILES_DIR` directory if it doesn't already exists.
    ///
    /// # Errors
    /// If `LOG_FILES_DIR` already exists but is not a directory.
    pub(super) fn ensure_logs_dir() -> Result<()> {
        if !LOG_FILES_DIR.try_exists()? {
            // If the logs path doesn't exist, create it.
            create_dir_all(&*LOG_FILES_DIR)?;
            info!("Created {}", LOG_FILES_DIR.display());
        } else if !LOG_FILES_DIR.is_dir() {
            // If the logs path does exist, but isn't a directory, bail.
            bail_log!(
                "{} exists but is not a directory. To avoid loss of data, it will not be deleted. Please move or delete it.",
                LOG_FILES_DIR.display()
            );
        } else {
            // The logs path is a directory that exists. No need to do anything.
        }

        Ok(())
    }
}
