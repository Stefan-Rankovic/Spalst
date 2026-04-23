//! SPDX-License-Identifier: GPL-3.0-only

use semver::Version;

/// The current update phase.
#[derive(Debug, Default)]
pub enum Phase {
    /// Pulling releases.
    #[default]
    CheckingForUpdates,
    /// Forcefully updating because the current release is marked unsafe.
    UpdatingFromUnsafe {
        /// What version the program is updating to.
        ///
        /// Should be the first safe release after the current one.
        to: Version,
    },
    /// Displaying a prompt for the user to choose whether they want to update or not.
    ShowingPrompt,
    /// Updating to the target version.
    Updating {
        /// What version the program is updating to.
        to: Version,
    },
    /// Updating to the latest version.
    UpdatingToLatest,
    /// Update process is over.
    /// Updated.
    Updated,
    /// Update process is over.
    /// Didn't update.
    DidNotUpdate,
    /// Update process is over.
    /// Did not do anything except `CheckingForUpdates` (that is required for the program to even
    /// know it is marked as unsafe).
    /// Continuing to the program normally.
    EarlyUnsafeUpdateExit,
    /// Update process is over.
    /// Did not do anything except `CheckingForUpdates` (that is required for the program to even
    /// know it is marked as unsafe).
    /// `panic!` will be invoked.
    EarlyUnsafeProgramExit,
}

impl Phase {
    /// Finish the update process.
    ///
    /// According to the current phase, `self` will change to `Updated` or `DidNotUpdate`.
    /// If `self` is already one of these, it will not change.
    ///
    /// If `self` is `EarlyUnsafeUpdateExit` or `EarlyUnsafeProgramExit`, it will not change.
    pub fn finish(&mut self) {
        if matches!(
            self,
            Self::Updated | Self::DidNotUpdate | Self::EarlyUnsafeUpdateExit | Self::EarlyUnsafeProgramExit
        ) {
            return;
        }
        *self = match *self {
            Self::CheckingForUpdates | Self::ShowingPrompt => Self::DidNotUpdate,
            Self::UpdatingFromUnsafe | Self::Updating { .. } | Self::UpdatingToLatest => Self::Updated,
            Self::Updated | Self::DidNotUpdate | Self::EarlyUnsafeUpdateExit | Self::EarlyUnsafeProgramExit => unreachable!("matches! handles these cases above. Getting this earns you a cookie 🍪"),
        }
    }

    /// Show the user prompt.
    ///
    /// If `self` is not `EarlyUnsafeUpdateExit` or `EarlyUnsafeProgramExit` (in which case it
    /// remains the same), it will be set to `ShowingPrompt`.
    pub fn set_to_prompt(&mut self) {
        if matches!(
            self,
            Self::EarlyUnsafeUpdateExit | Self::EarlyUnsafeProgramExit
        ) {
            return;
        }
        *self = Self::ShowingPrompt;
    }

    /// Update from unsafe.
    ///
    /// If `self` is not `EarlyUnsafeUpdateExit` or `EarlyUnsafeProgramExit` (in which case it
    /// remains the same), it will be set to `UpdatingFromUnsafe`.
    pub fn update_from_unsafe(&mut self) {
        if matches!(
            self,
            Self::EarlyUnsafeUpdateExit | Self::EarlyUnsafeProgramExit
        ) {
            return;
        }
        *self = Self::UpdatingFromUnsafe;
    }
}
