//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Updater;

impl Updater {
    /// Whether the program was safe on startup.
    ///
    /// As the name suggests, this function doesn't take into account whether the program updated to
    /// a safe release. If it did, this would still return `false`.
    pub async fn was_safe(&self) -> bool {
        self.releases
            .lock()
            .await
            .safety_level_of_current()
            .is_safe()
    }
}
