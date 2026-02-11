//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::PlaythroughsSortMethod,
    structs::{Playthrough, PlaythroughId, SortDescending, Sorter},
};
use chrono::{DateTime, Utc};
use core::cmp::Ordering;

impl Sorter {
    fn playthrough_cmp(
        p1: &Playthrough,
        p2: &Playthrough,
        method: PlaythroughsSortMethod,
    ) -> Ordering {
        match method {
            PlaythroughsSortMethod::Name => p1.name.cmp(&p2.name),

            PlaythroughsSortMethod::Playtime => p1.playtime.cmp(&p2.playtime),
            PlaythroughsSortMethod::SaveNumber => p1.saves.len().cmp(&p2.saves.len()),

            // Reversing the order of the comparison is intended.
            PlaythroughsSortMethod::CreatedAgo => p2.created_at.cmp(&p1.created_at),
            PlaythroughsSortMethod::LastPlayedAgo => {
                let mu: DateTime<Utc> = DateTime::<Utc>::MAX_UTC;
                p2.last_played_at
                    .unwrap_or(mu)
                    .cmp(&p1.last_played_at.unwrap_or(mu))
            }
        }
    }
    pub fn playthroughs_no_id(
        playthroughs: &mut [Playthrough],
        method: PlaythroughsSortMethod,
        descending: SortDescending,
    ) {
        playthroughs.sort_by(|p1, p2| Self::playthrough_cmp(p1, p2, method));
        if *descending {
            playthroughs.reverse();
        }
    }
    pub fn playthroughs_with_id(
        playthroughs: &mut [(&PlaythroughId, &Playthrough)],
        method: PlaythroughsSortMethod,
        descending: SortDescending,
    ) {
        playthroughs.sort_by(|tuple1, tuple2| Self::playthrough_cmp(tuple1.1, tuple2.1, method));
        if *descending {
            playthroughs.reverse();
        }
    }
}
