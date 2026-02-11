//! SPDX-License-Identifier: GPL-3.0-only

use core::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EntityId(u64);

impl Deref for EntityId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
