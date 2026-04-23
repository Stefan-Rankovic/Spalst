//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::VersionSafety;
use derive_new::new;
use nonempty::NonEmpty;
use octocrab::models::repos::Release;

/// A list of sorted `Release`s.
///
/// # Guarantees
/// There will always be at least one `Release` (type-level).
/// Every `Release` will have a known `published_at`.
/// `Release`s will be sorted, newest first.
/// Output of any function will stay the same.
#[derive(Debug, new)]
pub struct Releases {
    /// Should remain unchanged!
    pub(super) releases: NonEmpty<Release>,

    pub(super) safety: Vec<VersionSafety>,
}
