//! SPDX-License-Identifier: GPL-3.0-only

/// The status of the update attempt.
///
/// Doesn't include cases gotten from updateing from an unsafe release as there's no "outcome" to
/// even get.
/// The program either updated to a safe release, or it `bail`ed (that's enforced by the type
/// system).
#[derive(Clone, Copy, Debug)]
pub enum Outcome {
    /// The user chose to update.
    Updated,
    /// The user chose not to update.
    Skipped,
    /// There are no safe releases to update to.
    AlreadyOnLatest,
}

impl Outcome {
    /// Whether the outcome means the program was updated.
    #[must_use]
    pub const fn updated(self) -> bool {
        matches!(self, Self::Updated)
    }
}
