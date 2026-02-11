//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Asset;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawRelease {
    pub tag_name: String,
    pub body: Option<String>,
    pub assets: Vec<Asset>,
}
