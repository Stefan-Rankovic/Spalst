//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Duration;

impl Duration {
    pub fn display_ago(&self) -> String {
        self.display() + " ago"
    }
}
