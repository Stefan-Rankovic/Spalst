//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::structs::AppState;
use color_eyre::Report;

impl AppState {
    /// Set the handle error to `None` and return the previous value.
    pub const fn take_handle_error(&mut self) -> Option<Report> {
        self.handle_error.take()
    }

    /// Set the handle error to `None` and return the previous value.
    pub const fn take_render_error(&mut self) -> Option<Report> {
        self.render_error.take()
    }

    /// Sets the handle error to the passed `Report`.
    ///
    /// # Panics
    /// If the handle error is already set.
    pub(super) fn set_handle_error(
        &mut self,
        error: Report,
    ) {
        assert!(self.handle_error.is_none(), "Handle error is already set.");
        self.handle_error = Some(error);
    }

    /// Sets the render error to the passed `Report`.
    ///
    /// # Panics
    /// If the render error is already set.
    pub fn set_render_error(
        &mut self,
        error: Report,
    ) {
        assert!(self.render_error.is_none(), "Render error is already set.");
        self.render_error = Some(error);
    }
}
