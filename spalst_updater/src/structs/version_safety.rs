//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::SafetyLevel;
use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VersionSafety {
    pub version: Version,
    pub safety_level: SafetyLevel,
}
