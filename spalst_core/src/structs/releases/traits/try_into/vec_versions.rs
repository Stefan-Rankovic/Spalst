//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Release, Releases};
use color_eyre::eyre::Report;
use semver::Version;

impl TryFrom<Releases> for Vec<&Version> {
    type Error = Report;

    fn try_from(value: Releases) -> Result<Self, Self::Error> {
        // Ok.
        Ok(value.releases()?.iter().map(Release::version).collect())
    }
}

impl<'releases> TryFrom<&'releases Releases> for Vec<&'releases Version> {
    type Error = Report;

    fn try_from(value: &'releases Releases) -> Result<Self, Self::Error> {
        // Ok.
        Ok(value.releases()?.iter().map(Release::version).collect())
    }
}
