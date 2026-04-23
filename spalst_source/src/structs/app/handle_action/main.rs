//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::{AppEvent, action::Action},
    structs::App,
};
use tracing::{debug, instrument, trace};

impl App {
    /// Utility function to handle an `AppEvent`.
    #[instrument(skip(self))]
    pub async fn handle_app_event(
        &mut self,
        app_event: AppEvent,
    ) {
        debug!("Got AppEvent, passing to MenuThing...");
        for action in self.display.handle_event(app_event).await {
            self.handle_action(action).await;
        }
        // trace!("Finished handling Event. Got Vec<Action> {actions:?}.");
    }

    /// Execute related code according to the passed `Action`.
    ///
    /// Exists because a `MenuThing` can't conventionally access data, so this is a workaround.
    #[instrument(skip(self))]
    pub async fn handle_action(
        &mut self,
        action: Action,
    ) {
        match action {
            Action::DisplayManagerAction(_) => unreachable!("This was handled in DisplayManager::handle_event() already. If you get this, you deserve a cookie 🍪"),
            Action::UpdateAction(update_action) => self.handle_update_action(update_action).await,
        }
    }
}
