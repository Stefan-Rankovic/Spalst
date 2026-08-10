//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{
    enums::ScreenManagerRequest,
    structs::{AppState, ScreenId, ScreenManager},
};
use color_eyre::Result;
use std::sync::{Arc, Mutex};
#[cfg(feature = "logging")]
use tracing::instrument;

impl AppState {
    /// Handle a `ScreenManagerRequest`.
    #[cfg_attr(feature = "logging", instrument(skip(state)))]
    pub(super) fn handle_screen_manager_request(
        state: &Arc<Mutex<Self>>,
        request: ScreenManagerRequest,
    ) -> Result<()> {
        let screen_manager: &mut ScreenManager = &mut state.lock().unwrap().screen_manager;

        match request {
            ScreenManagerRequest::AddScreen { parent_id, screen } => screen_manager
                .add_screen(parent_id, screen)
                .map(|_: ScreenId| ()),
            ScreenManagerRequest::SelectScreen(screen_id) => screen_manager.select_screen(screen_id),
            ScreenManagerRequest::AddAndSelectScreen { parent_id, screen } => screen_manager
                .add_select_screen(parent_id, screen)
                .map(|_: ScreenId| ()),
            ScreenManagerRequest::Back => screen_manager.back().map(|_: ScreenId| ()),
        }
    }
}
