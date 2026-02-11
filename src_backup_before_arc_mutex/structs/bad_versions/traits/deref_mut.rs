//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::BadVersions;
use std::ops::DerefMut;

impl DerefMut for BadVersions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
