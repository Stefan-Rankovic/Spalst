//! SPDX-License-Identifier: GPL-3.0-only

use bytes::Bytes;
use color_eyre::eyre::{OptionExt as _, Result};
use octocrab::models::repos::{Asset, Release};

/// Get the executable from Github for the current OS and architecture.
pub async fn get_asset(release: &Release) -> Result<Bytes> {
    let asset: &Asset = release
        .assets
        .iter()
        .find(|asset| asset_matches_target_name(&asset.name))
        .ok_or_eyre("Could not find a matching asset for the current OS and release.")?;

    let bytes: Bytes = reqwest::get(asset.browser_download_url.as_str())
        .await?
        .bytes()
        .await?;

    // Ok.
    Ok(bytes)
}

/// Whether the asset name is the correct one for the current OS and architecture.
fn asset_matches_target_name(_name: &str) -> bool {
    todo!()
}
