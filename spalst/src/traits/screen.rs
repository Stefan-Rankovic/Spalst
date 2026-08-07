//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{enums::Request, structs::App};
use core::fmt::Debug;
use egui::Ui;

/// A screen that renders UI within the application.
pub trait Screen: Debug + Send + Sync {
    /// Render the screen.
    fn ui(
        &self,
        ui: &mut Ui,
        app: &App,
    ) -> Vec<Request>;
}
