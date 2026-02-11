//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    structs::{RawRelease, Release, UnsafeVersion},
    utils::fetch_unsafe_versions,
};
use color_eyre::eyre::{OptionExt as _, Result, eyre};
use reqwest::{Client, Response};
use semver::Version;

#[derive(Debug)]
pub struct Releases {
    pub(in crate::structs::releases) releases: Vec<Release>,
}

impl From<Vec<Release>> for Releases {
    fn from(releases: Vec<Release>) -> Self {
        Self::new(releases)
    }
}

impl Releases {
    /// Creates a new `Releases`.
    pub const fn new(releases: Vec<Release>) -> Self {
        Self { releases }
    }
    /// Latest `Release`, if any.
    pub fn latest(&self) -> Option<&Release> {
        self.releases.first()
    }
    /// Sorts the releases by their version, so the newest release is the first.
    pub fn sort_unstable_by_version(&mut self) {
        self.releases
            .sort_unstable_by(|r1, r2| r1.version().cmp(r2.version()));
    }
    /// Checks if the passed `Version` exists.
    pub fn check_version(&self, version: &Version) -> Result<()> {
        self.find_with_version(version)
            .map(|_| ())
            .ok_or_eyre(eyre!("The version {version} doesn't exist."))
    }
    /// Gets only safe versions.
    pub fn safe_versions(&self) -> Vec<&Release> {
        self.releases
            .iter()
            .filter(|release| release.is_safe())
            .collect()
    }
    /// Finds the release that has the given version.
    pub fn find_with_version(&self, version: &Version) -> Option<&Release> {
        self.releases
            .iter()
            .find(|release| *release.version() == *version)
    }
    /// Returns a vector of the versions.
    pub fn as_versions(&self) -> Vec<&Version> {
        self.releases.iter().map(Release::version).collect()
    }
    /// Returns the first safe release after the passed one.
    ///
    /// If the passed release is safe, it will return that one.
    pub fn first_safe_after(&self, initial: &Version) -> Option<&Release> {
        self.releases
            .iter()
            .skip_while(|release| *release.version() != *initial)
            .find(|release| release.is_safe())
    }
    /// Fetches the releases from the GitHub repository.
    ///
    /// Automatically sorts the releases.
    pub async fn fetch() -> Result<Self> {
        // Fetch into Self.
        let mut releases: Self = Self::raw_fetch().await?;
        // Sort the releases.
        releases.sort_unstable_by_version();
        // Ok.
        Ok(releases)
    }
    /// todo: log this function
    async fn raw_fetch() -> Result<Self> {
        let client: Client = Client::new();
        let mut all_raw_releases: Vec<RawRelease> = Vec::new();
        let mut page: usize = 1;
        loop {
            let api_url: String = format!(
                "https://api.github.com/repos/{GITHUB_REPO_OWNER}/{GITHUB_REPO_NAME}/releases?per_page=100&page={page}",
            );
            let response: Response = client
                .get(api_url)
                .header("User-Agent", "spalst_updater")
                .send()
                .await?;
            let raw_releases: Vec<RawRelease> = response.json().await?;
            if raw_releases.is_empty() {
                break;
            }
            all_raw_releases.extend(raw_releases);
            page += 1;
        }
        // Fetch unsafe releases.
        let unsafe_versions: Vec<UnsafeVersion> = fetch_unsafe_versions(
            &all_raw_releases
                .first()
                .ok_or_eyre(eyre!("The GitHub repository has no releases."))?
                .tag_name,
        )
        .await?;
        let releases: Self = (all_raw_releases, unsafe_versions).try_into()?;
        // Ok.
        Ok(releases)
    }
}
