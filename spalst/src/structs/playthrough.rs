//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{
    bail_log,
    enums::SaveEntry,
    structs::{SaveId, save::Save},
};
use color_eyre::eyre::{OptionExt as _, Result};
use indextree::{Arena, Node};
#[cfg(feature = "logging")]
use tracing::instrument;

/// A game `Playthrough`.
///
/// In the future, there may be a problem. When forking a save, the user should be forbidden
/// from playing it. Instead, they can only fork it once again and play that fork. This is to prevent
/// other children from being effectively disconnected from their parent. This is marked with a
/// "todo" so that in the future I don't forget this.
#[derive(Debug, Default)]
pub struct Playthrough {
    /// The first save.
    ///
    /// Also the only one without a parent.
    root_save: Option<SaveId>,

    /// Save storage.
    arena: Arena<SaveEntry>,
}

impl Playthrough {
    /// Get the `Node` which this `SaveId` points to as a reference.
    ///
    /// # Errors
    /// If the passed `SaveId` doesn't point to a save.
    #[cfg_attr(feature = "logging", instrument(skip(self)))]
    pub fn get_save_node(
        &self,
        id: SaveId,
    ) -> Result<&Node<SaveEntry>> {
        self.arena.get(*id).ok_or_eyre(format!(
            "Passed `SaveId` (with value {}) doesn't point to a `Node`.",
            *id
        ))
    }

    /// Get the `SaveEntry` which this `SaveId` points to as a reference.
    ///
    /// # Errors
    /// If the passed `SaveId` doesn't point to a save.
    #[cfg_attr(feature = "logging", instrument(skip(self)))]
    pub fn get_save(
        &self,
        id: SaveId,
    ) -> Result<&SaveEntry> {
        Ok(self.get_save_node(id)?.get())
    }

    /// Get the `Node` which this `SaveId` points to as a mutable reference.
    ///
    /// # Errors
    /// If the passed `SaveId` doesn't point to a save.
    #[cfg_attr(feature = "logging", instrument(skip(self)))]
    pub fn get_save_node_mut(
        &mut self,
        id: SaveId,
    ) -> Result<&mut Node<SaveEntry>> {
        self.arena.get_mut(*id).ok_or_eyre(format!(
            "Passed ID (with value {}) doesn't point to anything.",
            *id
        ))
    }

    /// Get the `SaveEntry` which this `SaveId` points to as a mutable reference.
    ///
    /// # Errors
    /// If the passed `SaveId` doesn't point to a save.
    #[cfg_attr(feature = "logging", instrument(skip(self)))]
    pub fn get_save_mut(
        &mut self,
        id: SaveId,
    ) -> Result<&mut SaveEntry> {
        Ok(self.get_save_node_mut(id)?.get_mut())
    }

    /// Make a root `Save`.
    ///
    /// # Errors
    /// If a root `Save` already exists.
    #[cfg_attr(feature = "logging", instrument)]
    pub fn make_root_node(&mut self) -> Result<SaveId> {
        if self.root_save.is_some() {
            bail_log!("Root save already exists.");
        }
        let root_id: SaveId = self.arena.new_node(SaveEntry::Active(Save::new())).into();
        self.root_save = Some(root_id);
        Ok(root_id)
    }

    /// Fork a `Save` from the one this `SaveId` points to.
    ///
    /// # Errors
    /// If this `SaveId` doesn't point to a save.
    /// If this `SaveId` points to a deleted save.
    #[cfg_attr(feature = "logging", instrument(skip(self)))]
    pub fn fork_save(
        &mut self,
        parent_id: SaveId,
    ) -> Result<SaveId> {
        let SaveEntry::Active(ref parent) = *self.get_save(parent_id)? else {
            bail_log!(
                "Passed ID (with value {}) points to a deleted save.",
                *parent_id
            )
        };
        let child_id: SaveId = self
            .arena
            .new_node(SaveEntry::Active(parent.mk_child()))
            .into();
        Ok(child_id)
    }

    /// Delete a `Save`.
    /// That is done by having the node point to `SaveEntry::Deleted` instead of
    /// `SaveEntry::Active`.
    /// The metadata of the save won't be deleted, just the game data.
    ///
    /// # Errors
    /// If this `SaveId` doesn't point to a save.
    /// If this `SaveId` points to a deleted save.
    #[cfg_attr(feature = "logging", instrument(skip(self)))]
    pub fn delete_save(
        &mut self,
        id: SaveId,
    ) -> Result<()> {
        let SaveEntry::Active(ref active_save) = *self.get_save(id)? else {
            bail_log!(
                "Passed ID (with value {}) already points to a deleted save.",
                *id
            )
        };
        *self.get_save_mut(id)? = SaveEntry::Deleted(active_save.into());
        Ok(())
    }
}
