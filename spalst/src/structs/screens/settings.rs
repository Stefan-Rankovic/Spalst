//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{
    enums::{Request, ScreenManagerRequest},
    structs::App,
    traits::Screen,
};
use egui::{InnerResponse, Response, Ui};

/// Settings screen.
#[derive(Debug)]
pub struct SettingsScreen;

impl Screen for SettingsScreen {
    fn ui(
        &self,
        ui: &mut Ui,
        _app: &App,
    ) -> Vec<Request> {
        let mut result: Vec<Request> = Vec::new();
        let _: InnerResponse<()> = ui.vertical_centered(|ui: &mut Ui| {
            let _: Response = ui.heading("Settings");
            let _: Response = ui.label("Feature not implemented yet.");
            if ui.button("Back").clicked() {
                result.push(Request::ScreenManager(ScreenManagerRequest::Back));
            }
        });
        result
    }
}
