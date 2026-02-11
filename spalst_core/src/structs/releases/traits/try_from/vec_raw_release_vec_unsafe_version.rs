//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::ReleaseSafety,
    structs::{RawRelease, Release, Releases, UnsafeVersion},
};
use color_eyre::eyre::{Report, Result};
use semver::Version;

impl TryFrom<(Vec<RawRelease>, Vec<UnsafeVersion>)> for Releases {
    type Error = Report;
    fn try_from(value: (Vec<RawRelease>, Vec<UnsafeVersion>)) -> Result<Self> {
        // Take ownership here to avoid cloning ReleaseSafeties later.
        let mut unsafe_versions: Vec<UnsafeVersion> = value.1;

        let mut releases: Vec<Release> = Vec::new();
        for raw_release in value.0 {
            let current_version: Version = raw_release.tag_name.parse()?;
            let safety: ReleaseSafety = unsafe_versions
                .iter()
                .position(|unsafe_version| unsafe_version.version == current_version)
                .map_or(ReleaseSafety::Safe, |idx| {
                    unsafe_versions.swap_remove(idx).safety
                });
            releases.push(Release::from_raw(current_version, safety, raw_release));
        }
        // Ok.
        Ok(Self { releases })
    }
}
