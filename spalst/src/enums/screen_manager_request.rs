//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{structs::ScreenId, traits::Screen};

/// A `Request` that a `ScreenManager` will complete.
#[derive(Debug)]
pub enum ScreenManagerRequest {
    /// Add a new `Screen`.
    AddScreen {
        /// The parent ID of the `Screen`.
        parent_id: ScreenId,
        /// The `Screen` to add.
        screen: Box<dyn Screen>,
    },
    /// Select the `Screen` with this `ScreenId`.
    SelectScreen(ScreenId),
    /// Add and select the `Screen`.
    AddAndSelectScreen {
        /// The parent ID of the `Screen`.
        parent_id: ScreenId,
        /// The `Screen` to add.
        screen: Box<dyn Screen>,
    },
    /// Go back to the previous `Screen`.
    Back,
}
