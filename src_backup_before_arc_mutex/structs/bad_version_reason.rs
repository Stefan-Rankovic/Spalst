//! SPDX-License-Identifier: GPL-3.0-only

use serde::Deserialize;
use std::{
    fmt::{self, Display, Formatter},
    ops::Deref,
};

#[derive(Debug, Deserialize)]
pub struct BadVersionReason(String);

impl Deref for BadVersionReason {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for BadVersionReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}
