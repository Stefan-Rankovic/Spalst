//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Releases;
use std::ops::DerefMut;

impl DerefMut for Releases {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
