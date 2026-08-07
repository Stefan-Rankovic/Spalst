//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::enums::ScreenManagerRequest;

/// Actions that a `Screen` can request after rendering.
#[derive(Debug)]
pub enum Request {
    /// See documentation of `ScreenManagerRequest`.
    ScreenManager(ScreenManagerRequest),
    /// Quit the application.
    Quit,
}
