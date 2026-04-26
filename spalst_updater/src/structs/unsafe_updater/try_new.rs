//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Releases, UnsafeUpdater};
use color_eyre::eyre::Result;

impl UnsafeUpdater {
    /// Try to get a new instance of `Self`.
    ///
    /// # Errors
    /// If fetching releases from Github fails.
    pub async fn try_new() -> Result<Self> {
        // Ok.
        Ok(Self {
            releases: Releases::fetch().await?,
        })
    }
}
