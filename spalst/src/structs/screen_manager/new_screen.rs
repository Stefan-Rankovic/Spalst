//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{
    structs::{EmptyScreen, ScreenId, ScreenManager},
    traits::Screen,
};
use color_eyre::eyre::{OptionExt as _, Result};
#[cfg(feature = "logging")]
use tracing::instrument;

impl ScreenManager {
    /// Add a new screen as a child of the screen the passed `ScreenId` points to.
    ///
    /// # Errors
    /// If the passed `ScreenId` doesn't point to a `Screen`.
    #[cfg_attr(feature = "logging", instrument)]
    pub fn add_screen(
        &mut self,
        parent_id: ScreenId,
        screen: Box<dyn Screen>,
    ) -> Result<ScreenId> {
        let id: ScreenId = self.next_id();

        self.screens.push(screen.into());
        self.path_from_root.push(id);

        self.get_screen_node_mut(parent_id)
            .ok_or_eyre(format!(
                "Passed parent ID with value {} doesn't point to a screen node.",
                *parent_id,
            ))?
            .add_child(id);

        Ok(id)
    }

    /// Add a new screen as a child of the screen the passed `ScreenId` points to, and display it.
    ///
    /// # Errors
    /// If the call to `self.add_screen()` fails.
    #[cfg_attr(feature = "logging", instrument)]
    pub fn add_select_screen(
        &mut self,
        parent_id: ScreenId,
        screen: Box<dyn Screen>,
    ) -> Result<ScreenId> {
        let id: ScreenId = self.add_screen(parent_id, screen)?;
        self.current_id = id;
        Ok(id)
    }

    /// Add a root `Screen`.
    pub(super) fn add_root_screen(&mut self) {
        let id: ScreenId = self.next_id();
        let empty_screen: Box<dyn Screen> = Box::new(EmptyScreen);
        self.screens.push(empty_screen.into());
        self.root_id = id;
        self.current_id = id;
        self.path_from_root.push(id);
    }
}
