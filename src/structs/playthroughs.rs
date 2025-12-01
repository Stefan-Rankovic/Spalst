//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{PlaythroughsSortBy as SortBy, Select},
    structs::{Playthrough, PlaythroughName},
};
use chrono::{DateTime, Utc};
use color_eyre::eyre::{OptionExt, Result, bail};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
use tokio::time::Duration;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Playthroughs(HashMap<PlaythroughName, Playthrough>);

impl Deref for Playthroughs {
    type Target = HashMap<PlaythroughName, Playthrough>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Playthroughs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Playthroughs {
    /// Gets a playthrough based on the one passed and the enum `Select` passed.
    ///
    /// # Arguments
    /// * `initial` - The original playthrough name the function uses as a standpoint.
    /// * `target` - The `Select` enum that dictates how the element to be returned is chosen.
    /// * `sort_by` - Because a HashMap is not sorted, this is needed to sort it.
    /// * `sort_ascending` - Because a HashMap is not sorted, this is needed to sort it.
    pub fn get_new_playthrough(
        &self,
        initial: &PlaythroughName,
        target: Select<PlaythroughName>,
        sort_by: SortBy,
        sort_ascending: bool,
    ) -> Result<PlaythroughName> {
        // Get the sorted Vector.
        let sorted: Vec<(&PlaythroughName, &Playthrough)> = self.sorted(sort_by, sort_ascending);
        // If target is Direct.
        if let Select::Direct(pn) = target {
            // If the playthrough name passed actually exists, return the corresponding
            // playthrough, otherwise bail!().
            if sorted
                .iter()
                .map(|(pn, _): &(&PlaythroughName, &Playthrough)| -> &PlaythroughName { pn })
                .contains(&pn)
            {
                // Ok.
                return Ok(sorted
                    .into_iter()
                    .find(|(cpn, _): &(&PlaythroughName, &Playthrough)| -> bool { **cpn == pn })
                    .unwrap()
                    .0
                    .clone());
            } else {
                bail!(
                    "Passed playthrough name {} is not present in the list of playthroughs.",
                    pn.0
                );
            };
        };
        // Position of the initial element.
        let current_pos: usize = sorted
            .iter()
            .position(|(pn, _): &(&PlaythroughName, &Playthrough)| -> bool { **pn == *initial })
            .ok_or_eyre(format!(
                "Passed current playthrough name {} is not present in the list of playthroughs.",
                initial.0
            ))?;
        // If the previous element should be returned but the current element is the first one,
        // return the last element.
        if Select::Previous == target && current_pos == 0 {
            return Ok(sorted[sorted.len() - 1].0.clone());
        };
        // If the next element should be returned but the current element is the last one, return
        // the first element.
        if Select::Next == target && current_pos == sorted.len() - 1 {
            return Ok(sorted[0].0.clone());
        };
        // Position of the new element.
        let new_position: usize = match target {
            Select::Next => current_pos + 1,
            Select::Previous => current_pos - 1,
            Select::Direct(_) => unreachable!(),
        };
        // Ok.
        Ok(sorted[new_position].0.clone())
    }
    pub fn sorted(
        &self,
        sort_by: SortBy,
        sort_ascending: bool,
    ) -> Vec<(&PlaythroughName, &Playthrough)> {
        let mut playthroughs_sorted: Vec<(&PlaythroughName, &Playthrough)> = match sort_by {
            SortBy::CreatedAt => self.iter().sorted_by_key(
                |(_, playthrough): &(&PlaythroughName, &Playthrough)| -> DateTime<Utc> {
                    playthrough.created_at
                },
            ),
            SortBy::LastPlayedAt => self.iter().sorted_by_key(
                |(_, playthrough): &(&PlaythroughName, &Playthrough)| -> DateTime<Utc> {
                    // Make sure that games not yet played show as if they are played in the
                    // future, not the past (end, not beginning, unlike the default for Option).
                    playthrough
                        .last_played_at
                        .unwrap_or(DateTime::<Utc>::MAX_UTC)
                },
            ),
            SortBy::Name => self.iter().sorted_by_key(
                |(playthrough_name, _): &(&PlaythroughName, &Playthrough)| -> &str {
                    &playthrough_name.0
                },
            ),
            SortBy::SaveNumber => self.iter().sorted_by_key(
                |(_, playthrough): &(&PlaythroughName, &Playthrough)| -> usize {
                    playthrough.saves.len()
                },
            ),
            SortBy::Playtime => self.iter().sorted_by_key(
                |(_, playthrough): &(&PlaythroughName, &Playthrough)| -> Duration {
                    playthrough.playtime
                },
            ),
        }
        .collect();
        if !sort_ascending {
            playthroughs_sorted.reverse();
        };

        playthroughs_sorted
    }
}
