//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::action::UpdateAction, structs::App};
use color_eyre::eyre::Result;
use tracing::instrument;

impl App {
    /// Execute related code according to the passed `UpdateAction`.
    #[instrument(skip(self))]
    pub async fn handle_update_action(
        &mut self,
        update_action: UpdateAction,
    ) -> Result<()> {
        match update_action {
            UpdateAction::StartFirstPhase => self.initialize_updater().await.map(|_| ()),
        }
    }
}
