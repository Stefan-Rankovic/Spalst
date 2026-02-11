//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{BadVersionReason, BadVersions};
use semver::Version;
use std::vec;

impl IntoIterator for BadVersions {
    type Item = (Version, BadVersionReason);
    type IntoIter = vec::IntoIter<(Version, BadVersionReason)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a> IntoIterator for &'a BadVersions {
    type Item = &'a (Version, BadVersionReason);
    type IntoIter = std::slice::Iter<'a, (Version, BadVersionReason)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'a> IntoIterator for &'a mut BadVersions {
    type Item = &'a mut (Version, BadVersionReason);
    type IntoIter = std::slice::IterMut<'a, (Version, BadVersionReason)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
