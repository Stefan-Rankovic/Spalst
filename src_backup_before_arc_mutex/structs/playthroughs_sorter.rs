//! SPDX-License-Identifier: GPL-3.0-only

use chrono::{DateTime, Utc};
use itertools::Itertools;

use crate::{
    enums::PlaythroughsSortMethod,
    structs::{Duration, Playthrough, PlaythroughId},
    traits::Sorter,
};

#[derive(Clone, Copy, Debug)]
pub struct PlaythroughsSorter {
    pub sort_method: PlaythroughsSortMethod,
    pub sort_ascending: bool,
}

impl PlaythroughsSorter {
    pub fn new(sort_method: PlaythroughsSortMethod, sort_ascending: bool) -> Self {
        Self {
            sort_method,
            sort_ascending,
        }
    }
}

impl<'playthrough> Sorter<PlaythroughsSortMethod> for PlaythroughsSorter {
    type Item = Playthrough<'playthrough>;

    fn sort_method(&self) -> &PlaythroughsSortMethod {
        &self.sort_method
    }
    fn sort_ascending(&self) -> &bool {
        &self.sort_ascending
    }

    fn sort_items<'items>(&self, items: &'items [Self::Item]) -> Vec<&'items Self::Item> {
        let mut playthroughs_sorted: Vec<&Playthrough> = match self.sort_method {
            PlaythroughsSortMethod::Name => items
                .iter()
                .sorted_by_key(|playthrough: &&Playthrough| -> &str { playthrough.name }),

            PlaythroughsSortMethod::Playtime => items
                .iter()
                .sorted_by_key(|playthrough: &&Playthrough| -> Duration { playthrough.playtime }),
            PlaythroughsSortMethod::SaveNumber => items
                .iter()
                .sorted_by_key(|playthrough: &&Playthrough| -> usize { playthrough.saves.len() }),

            PlaythroughsSortMethod::CreatedAt => {
                items
                    .iter()
                    .sorted_by_key(|playthrough: &&Playthrough| -> DateTime<Utc> {
                        playthrough.created_at
                    })
            }
            PlaythroughsSortMethod::LastPlayedAt => {
                items
                    .iter()
                    .sorted_by_key(|playthrough: &&Playthrough| -> DateTime<Utc> {
                        // Make sure that games not yet played show as if they are played in the
                        // future, not the past (end, not beginning, unlike the default for Option).
                        playthrough
                            .last_played_at
                            .unwrap_or(DateTime::<Utc>::MAX_UTC)
                    })
            }
        }
        .collect();

        if !self.sort_ascending {
            playthroughs_sorted.reverse();
        };

        playthroughs_sorted
    }
}
