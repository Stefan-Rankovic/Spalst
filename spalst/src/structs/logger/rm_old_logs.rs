//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{
    debug,
    info,
    statics::LOG_FILES_DIR,
    structs::{AbsolutePathBuf, Logger},
};
use color_eyre::eyre::{Ok, Result};
use futures::future::try_join_all;
use tokio::fs::{self, DirEntry, ReadDir};
use tracing::instrument;

impl Logger {
    /// Get the logfiles.
    ///
    /// # Errors
    /// If `tokio::fs::read_dir()` fails.
    /// If `tokio::fs::ReadDir::next_entry()` fails.
    async fn log_files(&self) -> Result<Vec<DirEntry>> {
        let mut entries: Vec<DirEntry> = Vec::new();

        let mut read_dir: ReadDir = fs::read_dir(&*LOG_FILES_DIR).await?;
        while let Some(dir_entry) = read_dir.next_entry().await? {
            entries.push(dir_entry);
        }

        // entries.sort_unstable_by_key(DirEntry::file_name);

        Ok(entries)
    }

    /// Deletes all logs, except the last one.
    #[instrument(skip(self))]
    pub async fn rm_old_logs(&self) -> Result<()> {
        let entries: Vec<DirEntry> = self.log_files().await?;

        let mut deletion_futures = Vec::with_capacity(entries.len());

        for entry in entries {
            let entry_path: AbsolutePathBuf = entry.path().try_into()?;
            if entry_path == self.log_file {
                info!(
                    "Skipping logfile {} because it is the current logfile.",
                    self.log_file.display()
                );
                continue;
            }
            deletion_futures.push(async move {
                fs::remove_file(entry_path).await?;
                debug!("Removed previous log {}", entry.path().display());
                Ok(())
            });
        }

        let _: Vec<()> = try_join_all(deletion_futures).await?;
        info!("Removed all previous logs.");

        Ok(())
    }
}
