//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::{Releases, SafetyLevel, VersionSafety},
    utils::{current_version, release_to_version},
};
use octocrab::models::repos::Release;
use semver::Version;

impl Releases {
    /// The safety level of the passed `Version`.
    #[must_use]
    pub fn safety_level_of_version(
        &self,
        version: &Version,
    ) -> SafetyLevel {
        self.safety
            .iter()
            .find(|current_version_safety: &&VersionSafety| current_version_safety.version == *version)
            .map_or(SafetyLevel::SAFE, |version_safety: &VersionSafety| {
                version_safety.safety_level
            })
    }

    /// The safety level of the passed `Release`.
    #[must_use]
    pub fn safety_level_of_release(
        &self,
        release: &Release,
    ) -> SafetyLevel {
        let version: Version = release_to_version(release);
        self.safety_level_of_version(&version)
    }

    /// The safety level of the current `Release`.
    #[must_use]
    pub fn safety_level_of_current(&self) -> SafetyLevel {
        self.safety_level_of_version(&current_version())
    }

    /// The safety level of the latest `Release`.
    ///
    /// todo: maybe remove this; I don't see how this is more useful than `latest_safe()`
    #[must_use]
    pub fn safety_level_of_latest(&self) -> SafetyLevel {
        self.safety_level_of_version(&self.latest_version())
    }

    /// The first safe `Release` after the current one.
    #[must_use]
    pub fn first_safe(&self) -> Option<&Release> {
        self.newer_safe().into_iter().next_back()
    }

    /// The latest safe `Release` after the current one.
    #[must_use]
    pub fn latest_safe(&self) -> Option<&Release> {
        self.newer_safe().into_iter().next()
    }

    /// All safe releases newer than the current one.
    ///
    /// Newest release is first.
    #[must_use]
    pub fn newer_safe(&self) -> Vec<&Release> {
        self.newer()
            .into_iter()
            .filter(|release: &&Release| self.safety_level_of_release(release).is_safe())
            .collect()
    }
}
