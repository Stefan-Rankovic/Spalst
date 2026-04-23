//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Updater;
use color_eyre::eyre::Result;

impl Updater {
    /// Update to the latest version.
    ///
    /// # Errors
    /// If `self.update_to_release()` fails (check its documentation for more information).
    ///
    /// # Panics
    /// If the current version is the latest.
    pub async fn update_to_latest(&self) -> Result<()> {
        assert!(
            !self.releases.lock().await.is_on_latest(),
            "This function should only be called if the current version is not the latest."
        );

        self.update_to_release(self.releases.lock().await.latest_release())
            .await
    }
}
