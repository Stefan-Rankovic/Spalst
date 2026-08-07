//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{structs::ScreenId, traits::Screen};

/// A node for a `Screen` in `ScreenManager`.
#[derive(Debug)]
pub struct ScreenNode {
    /// The value of the node.
    pub screen: Box<dyn Screen>,
    /// The parents of the node.
    parents: Vec<ScreenId>,
    /// The children of the node.
    children: Vec<ScreenId>,
}

impl ScreenNode {
    /// Add this `ScreenId` as a child.
    pub fn add_child(
        &mut self,
        child_id: ScreenId,
    ) {
        self.children.push(child_id);
    }

    /// Add this `ScreenId` as a parent.
    pub fn add_parent(
        &mut self,
        parent_id: ScreenId,
    ) {
        self.parents.push(parent_id);
    }
}

impl From<Box<dyn Screen>> for ScreenNode {
    fn from(screen: Box<dyn Screen>) -> Self {
        Self {
            screen,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }
}
