//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::structs::Save;
use chrono::{DateTime, Utc};
use core::time::Duration;

/// A deleted save.
///
/// Contains only metadata.
///
/// Deleting saves is troublesome. Its metadata is lost, and its children will then point to the
/// save's parent. Both of those aren't good.
/// That is why this exists. It preserves its status as a `Node` and it also saves metadata.
#[derive(Debug)]
pub struct DeletedSave {
    /// The playtime on this `Save`, before it was deleted. Does not include the playtime of its parent.
    playtime: Duration,

    /// The time when this `Save` was created from its parent `Save`.
    /// If it has no parent, this is the time when the `Playthrough` was created.
    birth_time: DateTime<Utc>,
}

impl From<&Save> for DeletedSave {
    fn from(save: &Save) -> Self {
        Self {
            playtime: save.get_playtime(),
            birth_time: save.get_birth_time(),
        }
    }
}
