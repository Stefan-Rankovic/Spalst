//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::structs::Logger;
use std::fs;
use tracing::{error, info, trace};

impl Drop for Logger {
    fn drop(&mut self) {
        trace!("Dropping Logger...");
        if !self.rm_on_drop {
            trace!("`rm_on_drop` was `false`. Logfile won't be deleted.");
            return;
        }
        if !self.successful_exit {
            trace!("`successful_exit` was `false`. Logfile won't be deleted even though `rm_on_drop` is `true`.");
            return;
        }
        info!(
            "While dropping `Logger`, both `rm_on_drop` and `successful_exit` were `true`. Deleting log file at {}...",
            self.log_file.display()
        );
        if let Err(error) = fs::remove_file(&self.log_file) {
            error!("Failed to remove current log file with error \"{error}\".");
        }
    }
}
