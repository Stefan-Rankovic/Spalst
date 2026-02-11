//! SPDX-License-Identifier: GPL-3.0-only

use std::cmp::Ordering;

use crate::structs::Playthrough;

impl Ord for Playthrough {
    fn cmp(&self, other: &Self) -> Ordering {}
}
