//! SPDX-License-Identifier: GPL-3.0-only

use core::{
    num::NonZeroU8,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct SelectAmount(NonZeroU8);

impl From<NonZeroU8> for SelectAmount {
    fn from(value: NonZeroU8) -> Self {
        Self(value)
    }
}

impl Deref for SelectAmount {
    type Target = NonZeroU8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SelectAmount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SelectAmount {
    pub const ONE: Self = Self(NonZeroU8::new(1).unwrap());
    pub const MULTIPLE: Self = Self(NonZeroU8::new(3).unwrap());
}
