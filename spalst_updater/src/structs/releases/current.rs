//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    structs::Releases,
    utils::{current_version, release_to_version},
};
use octocrab::models::repos::Release;
use semver::Version;

impl Releases {
    /// Gets the current `Release`.
    ///
    /// # Panics
    /// If the current version doesn't have a corresponding release.
    #[expect(
        clippy::expect_used,
        reason = "Unreachable unless I mess up with version numbers in Cargo.toml or Github."
    )]
    #[must_use]
    pub fn current_release(&self) -> &Release {
        let current_version: Version = current_version();
        self.releases
            .iter()
            .find(|release: &&Release| release_to_version(release) == current_version)
            .expect("The current version doesn't exist as a release. This probably means you're ahead of the master branch. If not, you get a cookie 🍪")
    }

    /// todo: maybe remove this; I don't see how this is useful (because safeties exist)
    #[must_use]
    pub fn is_on_latest(&self) -> bool {
        current_version() == self.latest_version()
    }

    /// All releases newer than the current one.
    ///
    /// Newest release is first.
    #[must_use]
    pub fn newer(&self) -> Vec<&Release> {
        let current_release: &Release = self.current_release();
        self.releases
            .iter()
            .take_while(|release: &&Release| **release != *current_release)
            .collect()
    }
}
