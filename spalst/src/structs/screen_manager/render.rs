//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{
    enums::Request,
    structs::{App, ScreenManager},
};
use color_eyre::eyre::{OptionExt as _, Result};
use egui::Ui;
#[cfg(feature = "logging")]
use tracing::instrument;

impl ScreenManager {
    /// Render the screen by calling `ui()` (provided by `Screen`) on the current screen.
    #[cfg_attr(feature = "logging", instrument(skip(ui)))]
    pub fn render(
        &self,
        ui: &mut Ui,
        app: &App,
    ) -> Result<Vec<Request>> {
        Ok(self
            .get_screen_node(self.current_id)
            .ok_or_eyre(format!(
                "The current_id ({}) doesn't point to a `Node`",
                *self.current_id
            ))?
            .screen
            .ui(ui, app))
    }
}
