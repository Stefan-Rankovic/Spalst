//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use core::ops::{Deref, DerefMut};
use indextree::NodeId;

/// A wrapper over `indextree::NodeId`.
#[derive(Clone, Copy, Debug)]
pub struct SaveId(NodeId);

impl Deref for SaveId {
    type Target = NodeId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SaveId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<NodeId> for SaveId {
    fn from(node_id: NodeId) -> Self {
        Self(node_id)
    }
}
