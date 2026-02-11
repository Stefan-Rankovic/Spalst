//! SPDX-License-Identifier: GPL-3.0-only

use core::{
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};
use lasso::{Key as _, Spur};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PlaythroughId(pub Spur);

impl Display for PlaythroughId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // just print the internal usize
        write!(f, "{}", self.0.into_usize())
    }
}

impl Deref for PlaythroughId {
    type Target = Spur;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlaythroughId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Spur> for PlaythroughId {
    fn from(spur: Spur) -> Self {
        Self(spur)
    }
}
