//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Updater;
use color_eyre::eyre::Result;
use spalst_utils::bail_log;
use tracing::instrument;

impl Updater {
    /// Update to the first safe release, if one.
    ///
    /// Returns `true` if it found a safe release to update to, otherwise `false`.
    ///
    /// # Errors
    /// If `self.update_to()` fails (check its documentation for more information).
    ///
    /// # Panics
    /// If the current version is safe.
    #[instrument(skip(self))]
    pub async fn update_to_safe(&self) -> Result<()> {
        assert!(
            !self
                .releases
                .lock()
                .await
                .safety_level_of_current()
                .is_safe(),
            "This function should only be called if the current version is unsafe."
        );

        if let Some(first_safe) = self.releases.lock().await.first_safe() {
            self.update_to_release(first_safe).await
        } else {
            bail_log!("No safe release to update to found.");
        }
    }
}
