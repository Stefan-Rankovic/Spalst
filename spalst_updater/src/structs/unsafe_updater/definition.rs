//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Releases;

/// The thing that updates the program.
///
/// Has no guarantees of the current release safety.
#[derive(Debug)]
pub struct UnsafeUpdater {
    /// The list of all Github releases. See `Releases` documentation for more information.
    pub(super) releases: Releases,
}
