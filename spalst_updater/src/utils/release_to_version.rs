//! SPDX-License-Identifier: GPL-3.0-only

use color_eyre::eyre::{Report, Result};
use octocrab::models::repos::Release;
use semver::Version;

/// Gets the `Version` of a given `Release`.
///
/// # Errors
/// If `Version::parse()` fails.
pub fn release_to_version_fallible(release: &Release) -> Result<Version> {
    // Ok.
    Ok(release.tag_name.parse()?)
}

/// Gets the `Version` of a given `Release`.
///
/// # Panics
/// If `Version::parse()` fails.
#[must_use]
pub fn release_to_version(release: &Release) -> Version {
    release_to_version_fallible(release).unwrap_or_else(|error: Report| {
        panic!(
            "Failed to parse given Release tag_name {} into a Version with error {error}",
            release.tag_name
        )
    })
}
