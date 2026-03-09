//! SPDX-License-Identifier: GPL-3.0-only

/// The status of migration for current release.
#[derive(Debug)]
pub enum ReleaseMigration {
    /// Migration doesn't exist for this version at all.
    None,
    /// Migration exists to this version, but not from.
    To,
    /// Migration exists from this version, but not to.
    From,
    /// Migration exists both to and from this version.
    Both,
}
