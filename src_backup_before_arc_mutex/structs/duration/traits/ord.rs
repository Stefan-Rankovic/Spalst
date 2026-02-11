//! SPDX-License-Identifier: GPL-3.0-only

use std::cmp::Ordering;

use crate::structs::Duration;

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nanoseconds_total().cmp(&other.nanoseconds_total())
    }
}
