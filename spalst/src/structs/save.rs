//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use chrono::{DateTime, Utc};
use core::{marker::PhantomData, time::Duration};
use derive_new::new;

/// A save.
///
/// Contains the world, along with useful metadata.
#[derive(Debug, new)]
pub struct Save {
    /// The playtime on this `Save`. Does not include the playtime of its parent.
    #[new(default)]
    playtime: Duration,

    /// The time when this `Save` was created from its parent `Save`.
    /// If it has no parent, this is the time when the `Playthrough` was created.
    #[new(value = "Utc::now()")]
    birth_time: DateTime<Utc>,

    /// Game info that doesn't exist yet.
    game_info: PhantomData<u8>,
}

impl Save {
    /// Get the playtime of this `Save`.
    pub const fn get_playtime(&self) -> Duration {
        self.playtime
    }

    /// Get the birth time of this `Save`.
    pub const fn get_birth_time(&self) -> DateTime<Utc> {
        self.birth_time
    }

    /// Make a child from this `Save` and return it.
    pub fn mk_child(&self) -> Self {
        Self {
            playtime: Duration::ZERO,
            birth_time: Utc::now(),
            game_info: self.game_info,
        }
    }
}
