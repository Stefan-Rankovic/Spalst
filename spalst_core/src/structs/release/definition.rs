//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::ReleaseSafety,
    structs::{Asset, RawRelease},
};
use color_eyre::eyre::{OptionExt as _, Report, eyre};
use semver::Version;
use serde::Deserialize;
use serde_json::Value;

/// todo: this got its `Clone` derive removed. maybe bring it back?
#[derive(Debug, Deserialize)]
pub struct Release {
    version: Version,
    safety: ReleaseSafety,
    tag_name: String,
    pub body: Option<String>,
    pub assets: Vec<Asset>,
}

impl Release {
    pub const fn new(
        version: Version,
        safety: ReleaseSafety,
        tag_name: String,
        body: Option<String>,
        assets: Vec<Asset>,
    ) -> Self {
        Self {
            version,
            safety,
            tag_name,
            body,
            assets,
        }
    }
    pub fn from_raw(version: Version, safety: ReleaseSafety, raw_release: RawRelease) -> Self {
        Self::new(
            version,
            safety,
            raw_release.tag_name,
            raw_release.body,
            raw_release.assets,
        )
    }
    pub const fn version(&self) -> &Version {
        &self.version
    }
    pub const fn safety(&self) -> &ReleaseSafety {
        &self.safety
    }
    pub const fn tag_name(&self) -> &str {
        self.tag_name.as_str()
    }
    pub fn is_safe(&self) -> bool {
        self.safety() == &ReleaseSafety::Safe
    }
}

/// Marks this release as `Safe` even if it's not!
impl TryFrom<&Value> for Release {
    type Error = Report;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let tag_name: String = value
            .get("tag_name")
            .ok_or_eyre(eyre!("tag_name property doesn't exist."))?
            .as_str()
            .ok_or_eyre(eyre!("tag_name property is not a &str."))?
            .to_string();
        let body: Option<String> = value.get("body").map(|val| {
            val.as_str()
                .ok_or_eyre(eyre!("tag_name property is not a &str."))
                .unwrap()
                .to_string()
        });
        let assets: Vec<Asset> = value
            .get("assets")
            .ok_or_eyre(eyre!("assets property doesn't exist."))?
            .as_array()
            .ok_or_eyre(eyre!("assets property is not an array."))?
            .iter()
            .map(|val| Asset::try_from(val).unwrap())
            .collect();
        // Ok.
        Ok(Self {
            version: tag_name.parse()?,
            safety: ReleaseSafety::Safe,
            tag_name,
            body,
            assets,
        })
    }
}
