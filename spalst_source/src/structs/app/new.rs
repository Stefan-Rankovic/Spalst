//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::MenuThingPinnedStatus,
    structs::{App, ArgsParser, DisplayManager, Logger, menu_thing::WelcomeMT},
    utils::game_directory,
};
use core::sync::atomic::AtomicU8;
use std::sync::Arc;
use tokio::time::Instant;
use tracing::instrument;

impl App {
    /// Makes a new instance of `App`.
    #[instrument]
    pub fn new(
        args: ArgsParser,
        logger: Logger,
    ) -> Self {
        Self {
            args,
            logger,

            parent_path: game_directory().into(),

            terminal: ratatui::init(),
            display: DisplayManager::from_menu_thing(Box::new(WelcomeMT::new(
                true,
                true,
                MenuThingPinnedStatus::NotPinned,
                Instant::now(),
            )))
            .with_default_screen_block()
            .0, // Enter welcome screen on startup

            event_listener: Self::event_listener(),

            tps: Arc::new(AtomicU8::from(0)),
        }
    }
}
