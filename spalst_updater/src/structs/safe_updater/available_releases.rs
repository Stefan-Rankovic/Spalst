//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::SafeUpdater;
use octocrab::models::repos::Release;

impl SafeUpdater {
    /// Releases available as a target for updating.
    #[must_use]
    pub fn available_releases(&self) -> Vec<&Release> {
        self.releases.newer_safe()
    }
}
