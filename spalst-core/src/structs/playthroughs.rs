//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::PlaythroughsSortMethod,
    structs::{Playthrough, PlaythroughId, SortDescending, Sorter},
};
use color_eyre::eyre::{OptionExt as _, Result};
use lasso::Rodeo;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, hash_map};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Playthroughs {
    playthroughs: HashMap<PlaythroughId, Playthrough>,
    interner: Rodeo, // todo: change this to use this
}

impl Playthroughs {
    pub fn get(&self, key: PlaythroughId) -> Option<&Playthrough> {
        self.playthroughs.get(&key)
    }
    pub fn is_empty(&self) -> bool {
        self.playthroughs.is_empty()
    }
    pub fn contains_key(&self, key: PlaythroughId) -> bool {
        self.playthroughs.contains_key(&key)
    }
    pub fn iter(&self) -> hash_map::Iter<'_, PlaythroughId, Playthrough> {
        self.playthroughs.iter()
    }
}

impl Playthroughs {
    /// Gets the name of the playthrough by its ID.
    pub fn get_name(&self, id: PlaythroughId) -> Option<&str> {
        self.interner.try_resolve(&id)
    }
    /// Adds a new pair of key and value to the list. Handles interning the new ID.
    pub fn insert(&mut self, name: &str, playthrough: Playthrough) -> Option<Playthrough> {
        let id = self.interner.get_or_intern(name).into();
        self.playthroughs.insert(id, playthrough)
    }
    /// Gets the ID of the playthrough by its name.
    pub fn get_id(&self, name: &str) -> Option<PlaythroughId> {
        self.interner.get(name).map(PlaythroughId)
    }

    pub fn sorted(
        &self,
        method: PlaythroughsSortMethod,
        descending: SortDescending,
    ) -> Vec<(&PlaythroughId, &Playthrough)> {
        let mut vec: Vec<(&PlaythroughId, &Playthrough)> = self.playthroughs.iter().collect();
        Sorter::playthroughs_with_id(&mut vec, method, descending);
        vec
    }

    /// Gets the index of the playthrough that has the passed ID.
    ///
    /// Sorts the inner `HashMap` according to the sort arguments passed.
    ///
    /// # Errors
    /// If the passed ID doesn't exist.
    pub fn get_index(
        &self,
        target: PlaythroughId,
        sort_method: PlaythroughsSortMethod,
        sort_descending: SortDescending,
    ) -> Result<usize> {
        self.sorted(sort_method, sort_descending)
            .iter()
            .position(|&(pid, _)| *pid == target)
            .ok_or_eyre(format!("ID {target} doesn't exist."))
    }

    /// Gets the ID of the element that follows the passed one (if there is one that follows).
    ///
    /// Sorts the inner `HashMap` according to the sort arguments passed.
    ///
    /// # Errors
    /// If the passed ID doesn't exist.
    pub fn get_next(
        &self,
        initial: PlaythroughId,
        sort_method: PlaythroughsSortMethod,
        sort_descending: SortDescending,
    ) -> Result<Option<PlaythroughId>> {
        // Ok.
        Ok(self
            .sorted(sort_method, sort_descending)
            .get(self.get_index(initial, sort_method, sort_descending)? + 1)
            .map(|&(pid, _)| *pid))
    }

    /// Gets the ID of the element that precedes the passed one (if there is one that precedes).
    ///
    /// Sorts the inner `HashMap` according to the sort arguments passed.
    ///
    /// # Errors
    /// If the passed ID doesn't exist.
    pub fn get_previous(
        &self,
        initial: PlaythroughId,
        sort_method: PlaythroughsSortMethod,
        sort_descending: SortDescending,
    ) -> Result<Option<PlaythroughId>> {
        Ok(self
            .sorted(sort_method, sort_descending)
            .get(self.get_index(initial, sort_method, sort_descending)? - 1)
            .map(|&(pid, _)| *pid))
    }
}
