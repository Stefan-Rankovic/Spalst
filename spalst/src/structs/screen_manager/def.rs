//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::structs::{ScreenId, ScreenNode};
use color_eyre::eyre::{OptionExt as _, Result};
#[cfg(feature = "logging")]
use tracing::instrument;

/// The manager for values that implement `Screen`, screens.
///
/// Simply having a `Vec<Box<dyn Screen>>` would work for display but wouldn't work
#[derive(Debug)]
pub struct ScreenManager {
    /// `Screen` collection.
    pub(super) screens: Vec<ScreenNode>,
    /// The path the user took to get from the root `Screen` to the current one.
    ///
    /// The first element will always be the root node.
    ///
    /// This is important because otherwise a "Back" button wouldn't be possible (since each node in
    /// `screens` can have multiple parents).
    ///
    /// When the user presses a "Back" button, the last element of this `Vec` will be removed.
    /// When they navigate back to an already-encountered screen (including the most recent one)
    /// without pressing a "Back" button, the last element of this `Vec` will not be removed, and
    /// instead a new element will be added.
    pub(super) path_from_root: Vec<ScreenId>,

    /// The first `Screen`.
    pub(super) root_id: ScreenId,
    /// The current `Screen`.
    pub(super) current_id: ScreenId,
}

impl ScreenManager {
    /// Returns the `ScreenId` of the root `Screen`.
    pub const fn root_id(&self) -> ScreenId {
        self.root_id
    }

    /// Returns the `ScreenId` of the current `Screen`.
    pub const fn current_id(&self) -> ScreenId {
        self.current_id
    }

    /// The next available `ScreenId`.
    pub(super) fn next_id(&self) -> ScreenId {
        self.screens.len().into()
    }

    /// Ensure that the passed ID is valid.
    ///
    /// # Errors
    /// If it isn't valid. In other words, if it doesn't point to a `ScreenNode`.
    #[cfg_attr(feature = "logging", instrument)]
    pub(super) fn ensure_valid_id(
        &self,
        id: ScreenId,
    ) -> Result<()> {
        self.screens
            .get(*id)
            .ok_or_eyre(format!(
                "Passed ID ({}) doesn't point to a `ScreenNode`.",
                *id
            ))
            .map(|_: &ScreenNode| ())
    }

    /// Get the `Node` of the given `ScreenId` as a reference, if one.
    pub fn get_screen_node(
        &self,
        id: ScreenId,
    ) -> Option<&ScreenNode> {
        self.screens.get(*id)
    }

    /// Get the `Node` of the given `ScreenId` as a mutable reference, if one.
    pub(super) fn get_screen_node_mut(
        &mut self,
        id: ScreenId,
    ) -> Option<&mut ScreenNode> {
        self.screens.get_mut(*id)
    }

    /// Get the `ScreenNode` of the current `Screen`.
    ///
    /// # Panics
    /// If the `current_id` doesn't point to a `ScreenNode`.
    pub fn get_current_screen_node(&self) -> &ScreenNode {
        self.screens.get(*self.current_id()).unwrap_or_else(|| {
            panic!(
                "The `current_id` ({}) doesn't point to a `ScreenNode`.",
                *self.current_id()
            )
        })
    }
}
