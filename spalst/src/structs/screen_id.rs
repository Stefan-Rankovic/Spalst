//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use core::ops::{Deref, DerefMut};

/// An ID for a `Screen` in `ScreenManager`.
#[derive(Clone, Copy, Debug)]
pub struct ScreenId(usize);

impl Deref for ScreenId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ScreenId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for ScreenId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
