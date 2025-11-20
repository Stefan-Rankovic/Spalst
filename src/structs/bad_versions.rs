//! SPDX-License-Identifier: GPL-3.0-only
use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    structs::{BadVersionReason, Release, Releases},
};
use color_eyre::eyre::{Context, Result, bail};
use reqwest::{Client, Response};
use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BadVersions(Vec<(Version, BadVersionReason)>);
impl BadVersions {
    /// Fetches the bad version from the GitHub repository.
    /// Automatically calls `self.check()` if the `releases` argument was passed.
    pub async fn fetch(release: &Release, releases: Option<&Releases>) -> Result<Self> {
        let client: Client = Client::new();
        let response: Response = client
            .get(format!(
                "https://raw.githubusercontent.com/{}/{}/{}/bad_versions.ron",
                GITHUB_REPO_OWNER, GITHUB_REPO_NAME, release.tag_name
            ))
            .send()
            .await
            .wrap_err_with(|| "Failed to download the bad_versions.ron file.")?;
        let text: String = response.text().await?;
        let parsed: Self = ron::from_str(&text)?;
        if let Some(releases) = releases {
            parsed.check(releases)?;
        };
        // Ok.
        Ok(parsed)
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
    pub fn check(&self, releases: &Releases) -> Result<()> {
        for (bad_version, _) in &self.0 {
            if releases.find_with_version(bad_version).is_none() {
                bail!("Bad version {bad_version} doesn't exist as a release.");
            };
        }
        // Ok.
        Ok(())
    }
}
