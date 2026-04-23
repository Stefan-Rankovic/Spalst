//! SPDX-License-Identifier: GPL-3.0-only

use serde::Deserialize;

/// How safe (or unsafe) a release is.
///
/// Unsafety can be (in no particular order):
///     - Bad migration code from the release (save corruption)
///     - Harmful update code
///     - Useless update code
///     - Always crashes
///     - Runtime save corruption
///
/// These alter the behaviour of the program. For more info, check any individual documentation for
/// a flag.
///
/// Having other significant bugs is not counted as unsafety because having bugs is almost guaranteed.
///
/// Note: 3 bit flags are currently unused.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SafetyLevel(u8);

impl From<u8> for SafetyLevel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl SafetyLevel {
    /// Bit access `const`.
    const BAD_MIGRATE_FROM: u8 = 1 << 0;
    /// Bit access `const`.
    const GUARANTEED_CRASH: u8 = 1 << 1;
    /// Bit access `const`.
    const HARMFUL_UPDATE: u8 = 1 << 2;
    /// Bit access `const`.
    const NO_UPDATE: u8 = 1 << 3;
    /// Bit access `const`.
    const RUNTIME_SAVE_CORRUPTION: u8 = 1 << 4;
    /// Completely safe release.
    pub const SAFE: Self = Self(0);

    /// The release is safe.
    ///
    /// Check happens by comparing `self` to `Self::SAFE`.
    #[must_use]
    pub fn is_safe(self) -> bool {
        self == Self::SAFE
    }

    /// Unsafe migration code from this release.
    /// Because migration code can be updated every new release, this should be temporary.
    ///
    /// Update code will immediately quit (to save time, as it will not update anyway).
    /// Otherwise, the program will run normally.
    ///
    /// The user can't update to or from this release automatically.
    /// It is advised against updating manually as that fixes nothing.
    #[must_use]
    pub const fn bad_migrate_from(self) -> bool {
        self.0 & Self::BAD_MIGRATE_FROM != 0
    }

    /// A crash that always happens.
    ///
    /// The user will be warned on startup.
    ///
    /// The user can't automatically update to this release. They can update from this release.
    #[must_use]
    pub const fn guaranteed_crash(self) -> bool {
        self.0 & Self::GUARANTEED_CRASH != 0
    }

    /// Update code will mess something up.
    ///
    /// Update code will immediately quit (to prevent it from being harmful).
    /// Otherwise, the program will run normally.
    ///
    /// The user can't update to or from this release automatically.
    #[must_use]
    pub const fn harmful_update(self) -> bool {
        self.0 & Self::HARMFUL_UPDATE != 0
    }

    /// Update code will not update.
    ///
    /// Update code will immediately quit (to save time).
    /// Otherwise, the program will run normally.
    ///
    /// The user can't update to or from this release automatically.
    #[must_use]
    pub const fn no_update(self) -> bool {
        self.0 & Self::NO_UPDATE != 0
    }

    /// The save can get corrupted.
    ///
    /// The program will attempt to update when it learns of this (since, as the release is not
    /// `harmful_update` so that must mean the save corruption happens after.
    ///
    /// The user can't update to, but can from, this release automatically.
    #[must_use]
    pub const fn runtime_save_corruption(self) -> bool {
        self.0 & Self::RUNTIME_SAVE_CORRUPTION != 0
    }

    /// Whether the update process should be skipped.
    #[must_use]
    pub const fn skip_update(self) -> bool {
        self.bad_migrate_from() || self.harmful_update() || self.no_update()
    }
}
