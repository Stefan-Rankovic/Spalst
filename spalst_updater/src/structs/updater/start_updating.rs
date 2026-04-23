//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::Outcome,
    structs::{SafetyLevel, Updater},
};
use color_eyre::eyre::Result;
use tracing::{error, info, instrument};

impl Updater {
    #[instrument(skip(self))]
    pub async fn start_updating(&self) -> Result<Outcome> {
        self.start_running()?;

        let safety_level_of_current: SafetyLevel = self.releases.lock().await.safety_level_of_current();

        if !safety_level_of_current.is_safe() {
            error!("Current release is unsafe!");
            if safety_level_of_current.skip_update() {
                error!("Unsafe level indicates updates should be skipped. Please update manually.");
                return Ok(Outcome::UnsafeSkip);
            }
            self.update_to_safe().await?;
        }

        self.phase.lock().await.set_to_prompt();

        self.stop_running()?;

        todo!()
    }
}
