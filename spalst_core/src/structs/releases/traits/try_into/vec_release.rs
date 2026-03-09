//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Release, Releases};
use color_eyre::eyre::{OptionExt, Report};

impl TryFrom<Releases> for Vec<Release> {
    type Error = Report;

    fn try_from(value: Releases) -> Result<Self, Self::Error> {
        // Ok.
        value
            .releases
            .into_inner()
            .ok_or_eyre("Struct not initialized.")
    }
}

impl<'releases> TryFrom<&'releases Releases> for &'releases Vec<Release> {
    type Error = Report;

    fn try_from(value: &'releases Releases) -> Result<Self, Self::Error> {
        value.releases.get().ok_or_eyre("Struct not initialized.")
    }
}

impl<'releases> TryFrom<&'releases mut Releases> for &'releases mut Vec<Release> {
    type Error = Report;

    fn try_from(value: &'releases mut Releases) -> Result<Self, Self::Error> {
        value
            .releases
            .get_mut()
            .ok_or_eyre("Struct not initialized.")
    }
}
