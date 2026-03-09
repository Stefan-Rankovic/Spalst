//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    structs::{RawRelease, Release, UnsafeVersion},
    utils::fetch_unsafe_versions,
};
use color_eyre::{
    Section,
    eyre::{OptionExt, Result, eyre},
};
use reqwest::{Client, Response};
use semver::Version;
use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct Releases {
    pub(in crate::structs::releases) releases: OnceCell<Vec<Release>>,
    current_release: OnceCell<usize>,
}

impl From<Vec<Release>> for Releases {
    fn from(releases: Vec<Release>) -> Self {
        Self {
            releases: releases.into(),
            current_release: OnceCell::new(),
        }
    }
}

impl Releases {
    pub fn new() -> Self {
        Self {
            releases: OnceCell::new(),
            current_release: OnceCell::new(),
        }
    }
}

impl Releases {
    pub const fn latest_index(&self) -> usize {
        0
    }
    pub fn latest(&self) -> Result<&Release> {
        self.releases()?.get(self.latest_index()).ok_or_eyre(
            "No releases exist. Getting this error is pretty impossible due to the earlier ?. If you get this, here's a cookie 🍪",
        )
    }
    pub fn current_index(&self) -> Result<usize> {
        let cargo_version: Version = env!("CARGO_PKG_VERSION")
            .parse()
            .with_note(|| "CARGO_PKG_VERSION doesn't have valid syntax.")?;
        self.releases()?
            .into_iter()
            .position(|release: &Release| *release.version() == cargo_version)
            .ok_or_eyre(format!(
                "CARGO_PKG_VERSION ({cargo_version}) doesn't exist as a version on GitHub."
            ))
    }
    /// Current release. `Err` if no releases exist.
    pub async fn current(&self) -> Result<&Release> {
        let releases: &Vec<Release> = self.releases()?;
        releases
            .get(
                *self
                    .current_release
                    .get_or_try_init(async || -> Result<usize> { self.current_index() })
                    .await?,
            )
            .ok_or_eyre("Invalid current_release cache.")
    }
    pub(in crate::structs::releases) fn releases(&self) -> Result<&Vec<Release>> {
        self.try_into()
    }
    pub(in crate::structs::releases) fn releases_mut(&mut self) -> Result<&mut Vec<Release>> {
        self.try_into()
    }
    /// Sorts the releases by their version, so the newest release is the first.
    ///
    /// # Errors
    /// If `self` isn't initialized.
    pub fn sort_unstable_by_version(&mut self) -> Result<()> {
        self.releases_mut()?
            .sort_unstable_by(|r1, r2| r1.version().cmp(r2.version()));
        // Ok.
        Ok(())
    }
    /// Checks if the passed `Version` exists.
    pub fn version_exists(&self, version: &Version) -> Result<bool> {
        if let Some(_) = self.find_version(version)?.map(|_| ()) {
            // Ok.
            Ok(true)
        } else {
            // Ok.
            Ok(false)
        }
    }
    /// Gets only safe versions.
    pub fn safe_versions(&self) -> Result<Vec<&Release>> {
        // Ok.
        Ok(self
            .releases()?
            .iter()
            .filter(|release| release.is_safe())
            .collect())
    }
    /// Finds the release that has the given version.
    pub fn find_version(&self, version: &Version) -> Result<Option<&Release>> {
        Ok(self
            .releases()?
            .iter()
            .find(|release| *release.version() == *version))
    }
    /// Returns a vector of the versions.
    pub fn as_versions(&self) -> Result<Vec<&Version>> {
        self.try_into()
    }
    /// Returns the first safe release after the passed one.
    ///
    /// If the passed release is safe, it will return that one.
    pub fn first_safe_after(&self, initial: &Version) -> Result<Option<&Release>> {
        Ok(self
            .releases()?
            .iter()
            .skip_while(|release| *release.version() != *initial)
            .find(|release| release.is_safe()))
    }
    /// Fetches the releases from the GitHub repository.
    ///
    /// Automatically sorts the releases.
    pub async fn fetch(self) -> Result<Self> {
        if let Some(_) = self.releases.get() {
            // Ok.
            return Ok(self);
        };
        self.releases.set(async || {
            // Fetch into Self.
            let mut releases: Self = Self::raw_fetch().await?;
            // Sort the releases.
            releases.sort_unstable_by_version();
            // Ok.
            Ok(releases)
        });
        // Ok.
        Ok(self)
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
