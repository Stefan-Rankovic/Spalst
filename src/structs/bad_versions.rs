/// SPDX-License-Identifier: GPL-3.0-only
use crate::structs::{BadVersionReason, Releases};
use color_eyre::eyre::{Result, bail};
use semver::Version;

#[derive(Debug)]
pub struct BadVersions(Vec<(Version, BadVersionReason)>);
impl BadVersions {
    /// Fetches the bad version from the GitHub repository.
    /// Automatically calls `self.check()` if the `releases` argument was passed.
    pub fn fetch(releases: Option<&Releases>) -> Result<Self> {
        todo!()
    }
    /// Checks if the passed version is one of the bad versions, and if it is, returns the reason.
    pub fn get_reason(&self, version: &Version) -> Option<&BadVersionReason> {
        self.0
            .iter()
            .find_map(|(bad_version, bad_version_reason): &(Version, BadVersionReason)| -> Option<&BadVersionReason> {
                (*bad_version == *version).then_some(bad_version_reason)
            })
    }
    /// Checks itself to see if the bad versions actually exist.
    pub fn check(&self, releases: Releases) -> Result<()> {
        for (bad_version, _) in &self.0 {
            if releases.find_with_version(bad_version).is_none() {
                bail!("Bad version {bad_version} doesn't exist as a release.");
            };
        }
        // Ok.
        Ok(())
    }
}
