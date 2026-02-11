//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Duration;
use std::cmp::Ordering;

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
