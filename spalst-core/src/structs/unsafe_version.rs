//! SPDX-License-Identifier: GPL-3.0-only

use crate::enums::ReleaseSafety;
use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UnsafeVersion {
    pub version: Version,
    pub safety: ReleaseSafety,
}
