//! SPDX-License-Identifier: GPL-3.0-only

use std::cmp::Ordering;

use crate::structs::Duration;

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
