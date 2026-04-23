//! SPDX-License-Identifier: GPL-3.0-only

#[derive(Clone, Copy, Debug)]
pub enum Outcome {
    /// Everything went normally.
    Normal,
    /// Current release is unsafe such that it doesn't permit attempting to update.
    UnsafeSkip,
}

impl Outcome {
    /// Whether the outcome means the program was updated.
    pub const fn updated(self) -> bool {
        matches!(self, Self::Normal)
    }
}
