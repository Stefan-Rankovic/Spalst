//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Duration;
use std::cmp::Ordering;

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nanoseconds_total().cmp(&other.nanoseconds_total())
    }
}
