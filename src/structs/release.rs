use crate::structs::Asset;
use color_eyre::eyre::{Context, Report, Result};
use semver::Version;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    name: Option<String>,
    pub body: Option<String>,
    published_at: Option<String>,
    html_url: String,
    assets: Vec<Asset>,
    //todo
}

impl TryFrom<Release> for Version {
    type Error = Report;
    fn try_from(release: Release) -> Result<Self> {
        Ok(release.tag_name.parse()?)
    }
}
impl TryFrom<&Release> for Version {
    type Error = Report;
    fn try_from(release: &Release) -> Result<Self> {
        Ok(release.tag_name.parse()?)
    }
}

impl Release {
    /// Gets the version for self.
    pub fn try_as_version(&self) -> Result<Version> {
        self.try_into()
    }
    /// Update to this release
    pub fn update_to(&self) -> Result<()> {
        todo!();
    }
}
