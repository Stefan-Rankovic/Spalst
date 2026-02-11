//! SPDX-License-Identifier: GPL-3.0-only

use core::ops::{Deref, Not};

#[derive(Copy, Clone, Debug)]
pub struct SortDescending(bool);

impl Deref for SortDescending {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Not for SortDescending {
    type Output = Self;

    fn not(self) -> Self::Output {
        (!self.0).into()
    }
}

impl From<bool> for SortDescending {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
