//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::structs::{DeletedSave, Save};

/// A save entry.
///
/// Holds either a `Save` or a `DeletedSave`.
///
/// To see why that's important, check the documentation of `DeletedSave`.
#[derive(Debug)]
pub enum SaveEntry {
    /// An active `Save`.
    Active(Save),
    /// A deleted save (`DeletedSave`).
    Deleted(DeletedSave),
}
