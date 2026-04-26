//! SPDX-License-Identifier: GPL-3.0-only

use crate::utils::get_asset;
use bytes::Bytes;
use color_eyre::eyre::Result;
use octocrab::models::repos::Release;

pub async fn update_to_release(target_release: &Release) -> Result<()> {
    let asset: Bytes = get_asset(target_release).await?;

    todo!("Make update code when there's a release.");
}
