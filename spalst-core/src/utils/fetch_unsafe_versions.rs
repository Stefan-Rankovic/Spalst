//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    structs::UnsafeVersion,
};
use color_eyre::eyre::{Context as _, Result};
use reqwest::{Client, Response};

/// Pulls unsafe versions from the GitHub repository.
pub async fn fetch_unsafe_versions(latest_release_tag_name: &str) -> Result<Vec<UnsafeVersion>> {
    let client: Client = Client::new();
    let response: Response = client
        .get(format!(
            "https://raw.githubusercontent.com/{GITHUB_REPO_OWNER}/{GITHUB_REPO_NAME}/{latest_release_tag_name}/unsafe_versions.ron"
        ))
        .send()
        .await
        .wrap_err_with(|| "Failed to download the unsafe_versions.ron file.")?;
    let text: String = response.text().await?;
    let unsafe_versions: Vec<UnsafeVersion> = ron::from_str(&text)?;
    // Ok.
    Ok(unsafe_versions)
}
