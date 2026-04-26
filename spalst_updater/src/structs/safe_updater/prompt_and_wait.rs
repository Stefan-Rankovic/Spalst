//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Updater;

impl Updater {
    pub(super) async fn prompt_and_wait(&self) {
        self.phase.lock().await.set_to_prompt();

        self.stopped_waiting_notifier.notified().await
    }
}
