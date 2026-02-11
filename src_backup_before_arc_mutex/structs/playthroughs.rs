//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::Select,
    structs::{Duration, Playthrough, PlaythroughId, PlaythroughName, PlaythroughsSorter},
    traits::Sorter,
};
use color_eyre::eyre::{OptionExt, Result, bail};
use itertools::Itertools;
use lasso::{Rodeo, Spur};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Playthroughs<'playthrough> {
    playthroughs: HashMap<PlaythroughId, Playthrough<'playthrough>>,
    interner: Rodeo, // todo: change this to use this
    sorter: PlaythroughsSorter,
}

impl Playthroughs<'_> {
    pub fn get(&self, k: &PlaythroughId) -> Option<&Playthrough> {
        self.playthroughs.get(k)
    }
    pub fn is_empty(&self) -> bool {
        self.playthroughs.is_empty()
    }
    pub fn contains_key(&self, k: &PlaythroughId) -> bool {
        self.playthroughs.contains_key(k)
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<PlaythroughId, Playthrough> {
        self.playthroughs.iter()
    }

    /// Prefer using Playthroughs::add_name() if possible.
    #[deprecated]
    pub fn insert(&mut self, k: PlaythroughId, v: Playthrough) -> Option<Playthrough> {
        self.playthroughs.insert(k, v)
    }
}

impl Playthroughs<'_> {
    /// Gets the name of the playthrough by its ID.
    pub fn get_name(&self, id: PlaythroughId) -> Option<&str> {
        self.interner.try_resolve(&id)
    }
    /// Adds a new pair of key and value to the list. Handles interning the new ID.
    pub fn add_name(&mut self, name: &str, playthrough: Playthrough) -> Option<Playthrough> {
        let id = PlaythroughId(self.interner.get_or_intern(name));
        self.playthroughs.insert(id, playthrough)
    }
    /// Gets the ID of the playthrough by its name.
    pub fn get_id(&self, name: &str) -> Option<PlaythroughId> {
        if let Some(id) = self.interner.get(&name) {
            Some(PlaythroughId(id))
        } else {
            None
        }
    }
    /// Gets a playthrough based on the one passed and the enum `Select` passed.
    ///
    /// # Arguments
    /// * `initial` - The original playthrough name the function uses as a standpoint.
    /// * `target` - The `Select` enum that dictates how the element to be returned is chosen.
    /// * `sort_by` - Because a HashMap is not sorted, this is needed to sort it.
    /// * `sort_ascending` - Because a HashMap is not sorted, this is needed to sort it.
    pub fn get_new_playthrough(
        &self,
        initial: PlaythroughId,
        target: Select<PlaythroughId>,
    ) -> Result<PlaythroughId> {
        // Get the sorted Vector.
        let sorted: Vec<(PlaythroughId, &Playthrough)> = self.sorted();
        // If target is Direct.
        if let Select::Direct(target_pid) = target {
            // If the playthrough name passed actually exists, return the corresponding
            // playthrough, otherwise bail!().
            if sorted
                .iter()
                .map(|(pid, _): &(PlaythroughId, &Playthrough)| -> PlaythroughId { *pid })
                .contains(&target_pid)
            {
                // Ok.
                return Ok(sorted
                    .into_iter()
                    .find(|(current_pid, _): &(PlaythroughId, &Playthrough)| -> bool {
                        *current_pid == target_pid
                    })
                    .unwrap()
                    .0
                    .clone());
            } else {
                bail!(
                    "Passed playthrough ID {} is not present in the list of playthroughs.",
                    target_pid
                );
            };
        };
        // Position of the initial element.
        let current_pos: usize = sorted
            .iter()
            .position(|(pid, _): &(PlaythroughId, &Playthrough)| -> bool { *pid == initial })
            .ok_or_eyre(format!(
                "Passed current playthrough name {} is not present in the list of playthroughs.",
                initial
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
    pub fn sorted(&self) -> Vec<(PlaythroughId, &Playthrough)> {
        self.sorter.sort_items(self.playthroughs);
    }
}
