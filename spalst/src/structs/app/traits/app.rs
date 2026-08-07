//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{enums::Request, structs::App};
use color_eyre::{Result, eyre::OptionExt as _};
use eframe::Frame;
use egui::{Ui, ViewportCommand};
use tracing::error;

impl eframe::App for App {
    fn ui(
        &mut self,
        ui: &mut Ui,
        _frame: &mut Frame,
    ) {
        // todo: add options for other themes as well (not just slate)
        elegance::Theme::slate().install(ui.ctx());

        let result: Result<()> = || -> Result<()> {
            let new_requests: Vec<Request> = self.state.lock().unwrap().screen_manager.render(ui, self)?;
            for request in new_requests {
                self.tx
                    .as_ref()
                    .ok_or_eyre("Sender dropped and `App::ui()` was called again")?
                    .send(request)?;
            }
            Ok(())
        }();

        // todo: close the app if this is true
        if let Err(report) = result {
            let error_message: String = format!("Encountered an error while rendering: {report}");
            error!(error_message);
            self.state.lock().unwrap().set_render_error(report);
            self.tx = None;
            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        }
    }
}
