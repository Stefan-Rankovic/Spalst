//! SPDX-License-Identifier: GPL-3.0-only

use core::{
    fmt::{self, Display, Formatter},
    ops::Deref,
};
use serde::Deserialize;

/// The reason for the unsafety of a release.
///
/// Wrapper over `String`.
#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReleaseUnsafetyReason(String);

impl Deref for ReleaseUnsafetyReason {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ReleaseUnsafetyReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", **self)
    }
}
