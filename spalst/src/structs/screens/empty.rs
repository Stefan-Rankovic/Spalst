//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{enums::Request, structs::App, traits::Screen};
use egui::Ui;

/// A completely empty screen.
#[derive(Debug)]
pub struct EmptyScreen;

impl Screen for EmptyScreen {
    fn ui(
        &self,
        _ui: &mut Ui,
        _app: &App,
    ) -> Vec<Request> {
        Vec::new()
    }
}
