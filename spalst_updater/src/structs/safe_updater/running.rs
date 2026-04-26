//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Updater;
use color_eyre::eyre::{Result, bail};
use std::sync::atomic::Ordering;

impl Updater {
    pub(super) fn start_running(&self) -> Result<()> {
        if self.running.load(Ordering::SeqCst) {
            bail!("This struct is already running.");
        };

        self.running.store(true, Ordering::SeqCst);

        // Ok.
        Ok(())
    }

    pub(super) fn stop_running(&self) -> Result<()> {
        if !self.running.load(Ordering::SeqCst) {
            bail!("This struct is not running.");
        };

        self.running.store(false, Ordering::SeqCst);

        // Ok.
        Ok(())
    }
}
