//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::{SafeUpdater, SafetyLevel, UnsafeUpdater},
    utils::update_to_release,
};
use color_eyre::eyre::Result;
use spalst_utils::bail_log;
use tracing::{debug, info, instrument};

impl UnsafeUpdater {
    /// Ensures the program is on a safe release.
    ///
    /// If it already is, passes a `Some(SafeProgramUpdater)`, otherwise `None`.
    /// That is because `CARGO_PKG_VERSION` remains from the old release.
    ///
    /// # Errors
    /// If the safety level of the current release forbids updating.
    /// If there's no safe release to update to (and the current release is unsafe).
    /// If updating to a safe release fails (and the current release is unsafe).
    #[instrument(skip(self))]
    pub async fn ensure_safe(self) -> Result<Option<SafeUpdater>> {
        let safety_of_current: SafetyLevel = self.releases.safety_level_of_current();

        debug!("Safety level of current release is {safety_of_current:?}");

        // If the current release is already safe, we have nothing more to do.
        if safety_of_current.is_safe() {
            debug!("Current release is already safe.");
            return Ok(Some(SafeUpdater::new(self.releases)));
        }

        // If the current release forbids updating, bail.
        if safety_of_current.skip_update() {
            bail_log!("Unsafety level of current release ({safety_of_current:?}) doesn't allow updating. Please update manually.");
        }

        // If there is a safe release to update to, do so.
        // Otherwise, bail.
        if let Some(first_safe) = self.releases.first_safe() {
            update_to_release(first_safe).await?;
        } else {
            bail_log!("No safe release to update to found.");
        }

        info!("Successfully updated to a safe release.");

        // We have successfully updated to a new release.
        // But, since the program was on an unsafe release, return None.
        Ok(None)
    }
}
