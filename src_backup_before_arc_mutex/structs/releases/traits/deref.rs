//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Release, Releases};
use std::ops::Deref;

impl Deref for Releases {
    type Target = Vec<Release>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
