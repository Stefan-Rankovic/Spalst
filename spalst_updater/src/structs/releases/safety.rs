//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::{Releases, SafetyLevel, VersionSafety},
    utils::{current_version, release_to_version},
};
use octocrab::models::repos::Release;
use semver::Version;

impl Releases {
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

    #[must_use]
    pub fn safety_level_of_release(
        &self,
        release: &Release,
    ) -> SafetyLevel {
        let version: Version = release_to_version(release);
        self.safety_level_of_version(&version)
    }

    #[must_use]
    pub fn safety_level_of_current(&self) -> SafetyLevel {
        self.safety_level_of_version(&current_version())
    }

    #[must_use]
    pub fn safety_level_of_latest(&self) -> SafetyLevel {
        self.safety_level_of_version(&self.latest_version())
    }

    #[must_use]
    pub fn first_safe(&self) -> Option<&Release> {
        let current: &Release = self.current_release();
        self.releases
            .iter()
            .skip_while(|release: &&Release| **release != *current)
            .find(|release: &&Release| self.safety_level_of_release(release).is_safe())
    }
}
