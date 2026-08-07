//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{enums::Request, structs::AppState};
use color_eyre::Result;
use std::sync::{Arc, Mutex};
use tracing::instrument;

impl AppState {
    /// Handle a `Request`.
    #[instrument(skip(state))]
    pub(in super::super) async fn handle_request(
        state: Arc<Mutex<Self>>,
        request: Request,
    ) -> Result<()> {
        match request {
            Request::Quit => todo!(),
            Request::ScreenManager(sm_request) => Self::handle_screen_manager_request(&state, sm_request),
        }
    }
}
