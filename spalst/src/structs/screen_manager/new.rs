//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::structs::ScreenManager;

impl ScreenManager {
    /// Construct a new `ScreenManager` with a root `Screen`.
    pub fn new() -> Self {
        let mut instance: Self = Self {
            screens: Vec::new(),
            root_id: usize::MAX.into(),
            current_id: usize::MAX.into(),
            path_from_root: Vec::new(),
        };

        instance.add_root_screen();

        instance
    }
}
