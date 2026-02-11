//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    structs::{Release, Releases, UnsafeVersion, UnsafeVersionReason},
};
use color_eyre::eyre::{Context as _, Result, bail};
use reqwest::{Client, Response};
use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UnsafeVersions {
    /// Immutable (since the API only provides an immutable reference to it) collection of all
    /// unsafe versions, together with the reason why each of them is marked as unsafe.
    unsafe_versions: Vec<UnsafeVersion>,
    /// Whether this collection has been checked.
    /// This can be treated as a guarantee that the unsafe versions actually exist, as long as the
    /// proper `Releases` instance was passed to `self.check()`.
    /// On the other hand, it is possible to trick the struct by passing an unrealistic `Releases`
    /// instance to `self.check()`. Or just creating the `UnsafeVersions` and saying it was already
    /// checked.
    checked: bool,
}

impl UnsafeVersions {
    /// Constructs a new `UnsafeVersions`.
    pub const fn new(unsafe_versions: Vec<UnsafeVersion>, checked: bool) -> Self {
        Self {
            unsafe_versions,
            checked,
        }
    }
    /// Returns all unsafe versions.
    pub fn unsafe_versions(&self) -> &Vec<UnsafeVersion> {
        &self.unsafe_versions
    }
    /// Fetches the unsafe version list from the GitHub repository.
    /// Automatically calls `self.check()` if the `releases` argument was passed.
    pub async fn fetch(release: &Release, releases: Option<&Releases>) -> Result<Self> {
        let client: Client = Client::new();
        let response: Response = client
            .get(format!(
                "https://raw.githubusercontent.com/{}/{}/{}/bad_versions.ron",
                GITHUB_REPO_OWNER,
                GITHUB_REPO_NAME,
                release.tag_name()
            ))
            .send()
            .await
            .wrap_err_with(|| "Failed to download the bad_versions.ron file.")?;
        let text: String = response.text().await?;
        let mut parsed: Self = ron::from_str(&text)?;
        if let Some(releases) = releases {
            parsed.check(releases)?;
        }
        // Ok.
        Ok(parsed)
    }
    /// Checks if the passed version is one of the unsafe versions, and if it is, returns the reason.
    pub fn get_reason(&self, version: &Version) -> Option<&UnsafeVersionReason> {
        self.unsafe_versions
            .into_iter()
            .find(|unsafe_version| *unsafe_version.version() == *version)
            .map(|unsafe_version| unsafe_version.reason())
    }
    /// Sets itself to unchecked.
    pub fn uncheck(&mut self) {
        self.checked = false;
    }
    /// Checks itself to see if the bad versions actually exist.
    pub fn check(&mut self, releases: &Releases) -> Result<()> {
        for (bad_version, _) in self {
            if releases.find_with_version(bad_version).is_none() {
                bail!("Bad version {bad_version} doesn't exist as a release.");
            }
        }
        self.checked = true;
        // Ok.
        Ok(())
    }
}
