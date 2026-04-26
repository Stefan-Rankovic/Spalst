//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::PromptInput, structs::Updater};
use std::sync::Arc;
use tokio::sync::Mutex;

impl Updater {
    pub fn prompt_input(&self) -> Arc<Mutex<PromptInput>> {
        Arc::clone(&self.prompt_input)
    }

    pub async fn set_prompt_input(
        &self,
        new_prompt_input: PromptInput,
    ) {
        let old_prompt_input: PromptInput = *self.prompt_input().lock().await;

        *self.prompt_input.lock().await = new_prompt_input;

        match new_prompt_input {
            PromptInput::NotAwaiting => unimplemented!("Setting PromptInput to NotAwaiting is not supported."),
            PromptInput::Awaiting => {
                if old_prompt_input != PromptInput::NotAwaiting {
                    unimplemented!("Setting PromptInput to Awaiting while the previous value was not NotAwaiting is not supported.");
                }
            }
            _ => self.stopped_waiting_notifier.notify_waiters(),
        }
    }
}
