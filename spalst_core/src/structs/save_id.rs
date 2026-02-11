//! SPDX-License-Identifier: GPL-3.0-only

use core::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SaveId(u64);

impl Deref for SaveId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SaveId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u64> for SaveId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}
