//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::structs::{ScreenId, ScreenManager};
use color_eyre::eyre::{OptionExt as _, Result};
#[cfg(feature = "logging")]
use tracing::instrument;

impl ScreenManager {
    /// Go back to the previous screen.
    ///
    /// # Errors
    /// If the current screen is the root screen.
    #[cfg_attr(feature = "logging", instrument)]
    pub fn back(&mut self) -> Result<ScreenId> {
        {
            self.current_id = *self
                .path_from_root
                .iter()
                .rev()
                .nth(1)
                .ok_or_eyre("Can't go back from root screen.")?;
        }
        let _: Option<ScreenId> = self.path_from_root.pop();
        Ok(self.current_id)
    }
}
