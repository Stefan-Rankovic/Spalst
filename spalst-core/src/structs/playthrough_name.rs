//! SPDX-License-Identifier: GPL-3.0-only

use core::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PlaythroughName(String);

impl Deref for PlaythroughName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlaythroughName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for PlaythroughName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", **self)
    }
}

impl From<String> for PlaythroughName {
    fn from(string: String) -> Self {
        Self(string)
    }
}
