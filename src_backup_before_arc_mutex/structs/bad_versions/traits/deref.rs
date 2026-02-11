//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{BadVersionReason, BadVersions};
use semver::Version;
use std::ops::Deref;

impl Deref for BadVersions {
    type Target = Vec<(Version, BadVersionReason)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
