//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::SafeUpdater;

impl SafeUpdater {
    /// Whether there is an available release to update to.
    ///
    /// Filters out releases that are unsafe.
    #[must_use]
    pub fn can_update(&self) -> bool {
        self.releases.first_safe().is_some()
    }
}
