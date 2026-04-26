//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER, SAFETY_PATH},
    structs::{Releases, VersionSafety},
};
use color_eyre::eyre::{OptionExt as _, Report, Result};
use core::cmp;
use nonempty::NonEmpty;
use octocrab::{Octocrab, Page, models::repos::Release};
use spalst_utils::bail_log;

impl Releases {
    /// Fetches releases from the GitHub repository.
    ///
    /// # Errors
    /// If `Octocrab::builder().build()` fails.
    /// If getting the pages from GitHub fails.
    /// If there are 0 releases.
    /// If any of the releases has no `published_at`.
    pub async fn fetch() -> Result<Self> {
        let crab: Octocrab = Octocrab::builder().build()?;

        let releases: NonEmpty<Release> = Self::fetch_releases(&crab).await?;

        let safety = Self::fetch_safety(&crab, &releases.head).await?;

        // Ok.
        Ok(Self::new(releases, safety))
    }

    async fn fetch_safety(
        crab: &Octocrab,
        latest_release: &Release,
    ) -> Result<Vec<VersionSafety>> {
        ron::from_str(
            &crab
                .repos(GITHUB_REPO_OWNER, GITHUB_REPO_NAME)
                .get_content()
                .path(SAFETY_PATH)
                .r#ref(&latest_release.tag_name)
                .send()
                .await?
                .items
                .into_iter()
                .next()
                .ok_or_eyre(format!("File {SAFETY_PATH} not found on GitHub."))?
                .decoded_content()
                .ok_or_eyre(format!(
                    "Failed to decode content of file {SAFETY_PATH} from GitHub."
                ))?,
        )
        .map_err(Report::from)
    }

    async fn fetch_releases(crab: &Octocrab) -> Result<NonEmpty<Release>> {
        let mut page: Page<Release> = crab
            .repos(GITHUB_REPO_OWNER, GITHUB_REPO_NAME)
            .releases()
            .list()
            .send()
            .await?;

        let mut releases: NonEmpty<Release> = if page.items.is_empty() {
            bail_log!("There are no releases on Github.");
        } else {
            let first: Release = page.items.swap_remove(0); // Fine because we sort later
            let rest: Vec<Release> = page.items;
            NonEmpty::from((first, rest))
        };

        while let Some(mut next_page) = crab.get_page(&page.next).await? {
            // Not `extend` because that would move `next_page` which can't be moved because of the
            // line after. Using `append` avoids that problem. It does drain `next_page.items`, but
            // we don't care about that.
            releases.append(&mut next_page.items);
            page = next_page;
        }

        // Guarantees every `Release` has a release date
        for release in &releases {
            if release.published_at.is_none() {
                bail_log!("Release {release:?} has published_at set to None.")
            }
        }

        // Sort by release date
        let mut releases_vec: Vec<Release> = releases.into();
        releases_vec.sort_unstable_by_key(|release: &Release| cmp::Reverse(release.published_at));
        let first: Release = releases_vec.remove(0);

        // Ok.
        Ok(NonEmpty::from((first, releases_vec)))
    }
}
