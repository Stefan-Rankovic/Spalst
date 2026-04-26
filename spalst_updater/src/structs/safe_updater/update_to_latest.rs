//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::Outcome, structs::SafeUpdater};
use color_eyre::eyre::Result;

impl SafeUpdater {
    /// Update to the latest safe release.
    ///
    /// # Errors
    ///
    /// # Panics
    /// If the current release is the latest.
    pub async fn update_to_latest(&self) -> Result<Outcome> {
        assert!(
            !self.releases.is_on_latest(),
            "This function should only be called if the current version is not the latest."
        );

        let Some(latest_safe) = self.releases.latest_safe() else {
            return Ok(Outcome::AlreadyOnLatest);
        };

        self.update_to_release(latest_safe).await?;

        // Ok.
        Ok(Outcome::Updated)
    }
}
