//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Release, Releases};
use std::vec;

impl IntoIterator for Releases {
    type Item = Release;
    type IntoIter = vec::IntoIter<Release>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> IntoIterator for &'a Releases {
    type Item = &'a Release;
    type IntoIter = std::slice::Iter<'a, Release>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'a> IntoIterator for &'a mut Releases {
    type Item = &'a mut Release;
    type IntoIter = std::slice::IterMut<'a, Release>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
