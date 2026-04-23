//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::AppEvent, structs::App, traits::Renderable as _};
use color_eyre::eyre::Result;
use ratatui::{CompletedFrame, Frame};
use tracing::{debug, instrument};

impl App {
    /// Run one game tick.
    #[instrument(skip(self))]
    pub async fn tick(&mut self) -> Result<()> {
        // While there are events, handle them.
        let mut got_event: bool = false;
        while let Ok(event) = self.event_listener.try_recv() {
            got_event = true;
            self.handle_app_event(AppEvent::Event(event)).await;
        }
        // If there were no events to handle, tick `MenuThing`.
        if !got_event {
            debug!("No Event in current tick. Passing AppEvent::Tick to MenuThing...");
            self.handle_app_event(AppEvent::Tick).await;
        }

        // todo: other things that need to tick.

        // Display.
        let _: CompletedFrame<'_> = self
            .terminal
            .draw(|frame: &mut Frame<'_>| self.display.basic_render(frame.area(), frame.buffer_mut()))?;

        // Ok.
        Ok(())
    }
}
