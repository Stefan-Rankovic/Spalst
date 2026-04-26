//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{Outcome, PromptInput},
    structs::{SafeUpdater, SafetyLevel},
};
use color_eyre::eyre::Result;
use tracing::{error, instrument};

impl SafeUpdater {
    #[instrument(skip(self))]
    pub async fn start_updating(&self) -> Result<Outcome> {
        self.start_running()?;

        if self.releases.is_on_latest() {
            self.stop_running()?;
            return Ok(Outcome::AlreadyOnLatest);
        };

        self.prompt_and_wait().await;

        let outcome: Outcome = match *self.prompt_input.lock().await {
            PromptInput::NotAwaiting | PromptInput::Awaiting => unreachable!("There should be valid response after prompt_and_wait() was called."),
            PromptInput::DontUpdate => Outcome::ChosenSkip,
            PromptInput::Update { ref to } => {
                self.update_to_release(to).await?;
                Outcome::ChosenUpdate
            }
        };

        self.stop_running()?;

        // Ok.
        Ok(outcome)
    }
}
