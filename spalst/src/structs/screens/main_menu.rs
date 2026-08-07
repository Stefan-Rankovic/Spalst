//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{
    enums::{Request, ScreenManagerRequest},
    structs::{App, screens::SettingsScreen},
    traits::Screen,
};
use egui::{InnerResponse, Response, Ui};
use elegance::Button;

/// The main menu of the program.
#[derive(Debug)]
pub struct MainMenuScreen;

impl Screen for MainMenuScreen {
    fn ui(
        &self,
        ui: &mut Ui,
        app: &App,
    ) -> Vec<Request> {
        let mut requests: Vec<Request> = Vec::new();
        let _: InnerResponse<()> = ui.vertical_centered(|ui: &mut Ui| {
            let _: Response = ui.heading("Spalst");
            {
                let response: Response = ui.add(Button::new("Continue").enabled(false));
                let _: Response = response.on_disabled_hover_text("Not implemented yet.");
            }
            {
                let response: Response = ui.add(Button::new("New Playthrough").enabled(false));
                let _: Response = response.on_disabled_hover_text("Not implemented yet.");
            }
            {
                let response: Response = ui.add(Button::new("Manage Playthroughs").enabled(false));
                let _: Response = response.on_disabled_hover_text("Not implemented yet.");
            }
            {
                let response: Response = ui.add(Button::new("Achievements").enabled(false));
                let _: Response = response.on_disabled_hover_text("Not implemented yet.");
            }
            if ui.button("Settings").clicked() {
                requests.push(Request::ScreenManager(
                    ScreenManagerRequest::AddAndSelectScreen {
                        screen: Box::new(SettingsScreen),
                        parent_id: app.state.lock().unwrap().screen_manager.current_id(),
                    },
                ));
            }
            {
                let response: Response = ui.add(Button::new("Check for Updates").enabled(false));
                let _: Response = response.on_disabled_hover_text("Not implemented yet.");
            }
            if ui.button("Quit").clicked() {
                requests.push(Request::Quit);
            }
        });
        requests
    }
}
