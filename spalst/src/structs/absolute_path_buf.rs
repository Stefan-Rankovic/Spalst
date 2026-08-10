//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::bail_log;
use color_eyre::eyre::{Report, Result};
use core::ops::Deref;
use std::path::{Path, PathBuf};
#[cfg(feature = "logging")]
use tracing::instrument;

/// Same as a `PathBuf`, except this is guaranteed to be an absolute path.
#[derive(Debug, Eq, PartialEq)]
pub struct AbsolutePathBuf(PathBuf);

impl AbsolutePathBuf {
    /// Constructs a new `AbsolutePathBuf` instance.
    ///
    /// # Errors
    /// If the passed `PathBuf` isn't absolute.
    #[cfg_attr(feature = "logging", instrument)]
    pub fn try_new(path_buf: PathBuf) -> Result<Self> {
        if !path_buf.is_absolute() {
            bail_log!(
                "Passed `PathBuf` with value \"{}\" isn't absolute.",
                path_buf.display()
            )
        }
        Ok(Self(path_buf))
    }

    /// You should probably use `AbsolutePathBuf::try_new()` instead.
    ///
    /// Constructs a new `AbsolutePathBuf` instance.
    ///
    /// # Panics
    /// If the passed `PathBuf` isn't absolute.
    pub fn new(path_buf: PathBuf) -> Self {
        assert!(
            path_buf.is_absolute(),
            "Passed `PathBuf` with value \"{}\" isn't absolute.",
            path_buf.display()
        );

        Self(path_buf)
    }

    /// A wrapper over `PathBuf::join`. This does the exact same thing as that (except this returns
    /// an `AbsolutePathBuf`).
    pub fn join<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Self {
        Self::new(self.0.join(path))
    }
}

impl Deref for AbsolutePathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<PathBuf> for AbsolutePathBuf {
    type Error = Report;

    fn try_from(path_buf: PathBuf) -> Result<Self> {
        Self::try_new(path_buf)
    }
}

impl AsRef<Path> for AbsolutePathBuf {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}
