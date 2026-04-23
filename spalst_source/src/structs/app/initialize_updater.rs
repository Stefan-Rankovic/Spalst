//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::App;
use color_eyre::eyre::Result;
use spalst_updater::structs::ProgramUpdater;
use tracing::instrument;

impl App {
    /// Initializes `self.updater` so it is `Some`.
    ///
    /// Returns whether it was already initialized.
    ///
    /// # Errors
    /// If `ProgramUpdater::try_new()` fails (check its documentation for more information).
    #[instrument(skip(self))]
    pub async fn initialize_updater(&mut self) -> Result<bool> {
        let already_initialized: bool = self.updater.is_some();

        if self.updater.is_none() {
            self.updater = Some(ProgramUpdater::try_new().await?);
        }

        // Ok.
        Ok(already_initialized)
    }
}
