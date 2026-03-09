//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Release, Releases};
use color_eyre::eyre::{Report, Result};
use semver::Version;

impl Releases {
    /// Returns a new `Vec<&Release>` that contains all releases newer than the passed `Version`.
    pub fn newer_than<'version, V>(&self, version: V) -> Result<Vec<&Release>>
    where
        V: TryInto<&'version Version>,
        V::Error: Into<Report>,
    {
        let ver: &'version Version = version.try_into().map_err(Into::into)?;
        // Ok.
        Ok(self
            .releases()?
            .iter()
            .take_while(|release| *release.version() != *ver)
            .collect())
    }
    /// Returns a new `Releases` that contains all releases newer than the passed `Version`.
    pub fn into_newer_than<'version, T>(self, version: T) -> Result<Self>
    where
        T: TryInto<&'version Version>,
        T::Error: Into<Report>,
    {
        let ver: &Version = version.try_into().map_err(Into::into)?;
        Ok(Vec::<Release>::try_from(self)?
            .into_iter()
            .take_while(|release| *release.version() != *ver)
            .collect::<Vec<_>>() // todo: replace the underscore with a type, regardless of whether this is an error
            .into())
    }
}
