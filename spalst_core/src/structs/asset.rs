//! SPDX-License-Identifier: GPL-3.0-only

use color_eyre::eyre::{OptionExt as _, Report, eyre};
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
    size: u64,
}

impl TryFrom<&Value> for Asset {
    type Error = Report;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let name: String = value
            .get("name")
            .ok_or_eyre(eyre!("name property doesn't exist."))?
            .as_str()
            .ok_or_eyre(eyre!("name property is not a &str."))?
            .to_string();
        let browser_download_url: String = value
            .get("browser_download_url")
            .ok_or_eyre(eyre!("browser_download_url property doesn't exist."))?
            .as_str()
            .ok_or_eyre(eyre!("browser_download_url property is not a &str."))?
            .to_string();
        let size: u64 = value
            .get("size")
            .ok_or_eyre(eyre!("size property doesn't exist."))?
            .as_u64()
            .ok_or_eyre(eyre!("browser_download_url property is not a u64."))?;
        // Ok.
        Ok(Self {
            name,
            browser_download_url,
            size,
        })
    }
}
