//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::ReleaseUnsafetyReason;
use serde::Deserialize;

/// How unsafe a certain release is.
///
/// Affects self-update behaviour when this release is involved.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum ReleaseSafety {
    /// Relatively safe.
    ///
    /// The only safety level that has migration code both to and from the release.
    ///
    /// Everything should work as normal.
    Safe,
    /// The self-update code is marked as unsafe.
    ///
    /// Migration code exists from this release, but not to.
    ///
    /// The user will be notified at every startup that self-update is disabled and that they
    /// should watch out for newer releases themselves.
    /// Everything else should work normally.
    UpdateUnsafe { reason: ReleaseUnsafetyReason },
    /// Generally unsafe.
    ///
    /// Migration code exists from this release, but not to.
    ///
    /// At every startup, an attempt to update to the first safe release available will be made. If
    /// that's not possible, the user will be prompted to exit or continue.
    /// Self-update should work as normal.
    Unsafe { reason: ReleaseUnsafetyReason },
    /// Really unsafe.
    ///
    /// The only safety level to have no migration code associated with it at all.
    ///
    /// At every startup, the user will be notified of this, and the program will
    /// exit immediately.
    ReallyUnsafe { reason: ReleaseUnsafetyReason },
}

impl ReleaseSafety {
    pub const fn reason(&self) -> Option<&ReleaseUnsafetyReason> {
        #[expect(
            clippy::pattern_type_mismatch,
            reason = "messes up the types if self is dereferenced or Self::X is borrowed in matching"
        )]
        match self {
            Self::Safe => None,
            Self::UpdateUnsafe { reason }
            | Self::Unsafe { reason }
            | Self::ReallyUnsafe { reason } => Some(reason),
        }
    }
}
