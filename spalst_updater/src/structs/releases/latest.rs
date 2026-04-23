//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::Releases, utils::release_to_version};
use octocrab::models::repos::Release;
use semver::Version;

impl Releases {
    #[must_use]
    pub const fn latest_release(&self) -> &Release {
        self.releases.first()
    }

    /// Gets the latest version.
    #[must_use]
    pub fn latest_version(&self) -> Version {
        release_to_version(self.latest_release())
    }
}
