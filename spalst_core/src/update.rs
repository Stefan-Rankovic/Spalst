//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    consts::{GITHUB_REPO_NAME, GITHUB_REPO_OWNER},
    structs::{Release, Releases},
    utils::epr_wrn,
};
use color_eyre::{
    Section as _,
    eyre::{OptionExt as _, Result, eyre},
};
use std::env;

pub async fn updater() -> Result<()> {
    let releases: Releases = Releases::fetch().await?;

    let current_release: &Release = releases
        .find_with_version(
            &env!("CARGO_PKG_VERSION")
                .parse()
                .with_note(|| "CARGO_PKG_VERSION doesn't have valid syntax.")?,
        )
        .ok_or_eyre(eyre!("CARGO_PKG_VERSION isn't a real version on GitHub."))?;

    let latest_release: Release = releases
        .latest()
        .ok_or_eyre(eyre!(
            "The repository {GITHUB_REPO_OWNER}/{GITHUB_REPO_NAME} has no releases."
        ))?
        .clone();

    if *current_release.version() > *latest_release.version() {
        epr_wrn("The program is on a version newer than the latest released version.");
        // Ok.
        return Ok(());
    } else if !current_release.is_safe() {
        handle_unsafe_release(current_release, latest_release, releases).await?;
    } else if *current_release.version() < *latest_release.version() {
        handle_update_to(current_release, latest_release, releases).await?;
    } else {
        // The current version is safe and also the latest one. Perfect!
    }

    // Ok.
    Ok(())
}

pub async fn handle_update_to(
    current_version: Release,
    latest_release: Release,
    releases: Releases,
) -> Result<()> {
    todo!()
}

pub async fn handle_unsafe_release(
    current_version: Release,
    latest_release: Release,
    releases: Releases,
) -> Result<()> {
    todo!()
}
