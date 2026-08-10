//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::structs::{ScreenId, ScreenManager};
use color_eyre::eyre::Result;
#[cfg(feature = "logging")]
use tracing::instrument;

impl ScreenManager {
    /// Select the `Screen` with the passed `ID`.
    ///
    /// # Errors
    /// If the passed ID doesn't point to a `ScreenNode`.
    #[cfg_attr(feature = "logging", instrument)]
    pub fn select_screen(
        &mut self,
        id: ScreenId,
    ) -> Result<()> {
        self.ensure_valid_id(id)?;
        self.current_id = id;
        Ok(())
    }
}
