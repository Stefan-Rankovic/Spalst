//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

#[cfg(feature = "logging")]
use crate::structs::Logger;
use crate::structs::{ArgsParser, MainMenuScreen, ScreenId, ScreenManager};
use clap::Parser as _;
use color_eyre::{Report, eyre::Result};

/// The actual state of the app.
///
/// This is separate from `App` because `App` is dropped after `eframe::run_native` finishes.
/// And, as also commented in the `render_error` field of this struct, that is a problem.
#[derive(Debug)]
pub struct AppState {
    /// The error encountered while handling requests.
    pub(super) handle_error: Option<Report>,
    /// The error encountered while rendering requests.
    ///
    /// Located here because `App` is dropped when `eframe::run_native` is called (inside `main()`).
    /// So extracting the error from `App` is impossible.
    pub(super) render_error: Option<Report>,

    /// CLI arguments passed to the program.
    args: ArgsParser,
    /// The logger.
    #[cfg(feature = "logging")]
    pub logger: Option<Logger>,
    /// The `Screen` manager.
    pub screen_manager: ScreenManager,
}

impl AppState {
    /// Tries to get a new `App` instance.
    ///
    /// # Errors
    /// If initializing the `Logger` fails (`Logger::try_init_new` function). Exclusive to the
    /// `logging` feature.
    #[cfg_attr(
        not(feature = "logging"),
        expect(clippy::unused_async, reason = "Only necessary to initialize Logger.")
    )]
    #[cfg_attr(
        not(feature = "logging"),
        expect(
            clippy::unused_async_trait_impl,
            reason = "Only necessary to initialize Logger."
        )
    )]
    pub async fn try_new() -> Result<Self> {
        let args: ArgsParser = ArgsParser::parse();
        #[cfg(feature = "logging")]
        let logger: Option<Logger> = args.log.then_some(Logger::try_init_new(&args).await?);
        let mut screen_manager: ScreenManager = ScreenManager::new();
        let _: ScreenId = screen_manager.add_select_screen(screen_manager.root_id(), Box::new(MainMenuScreen))?;

        Ok(Self {
            handle_error: None,
            render_error: None,
            args,
            #[cfg(feature = "logging")]
            logger,
            screen_manager,
        })
    }
}
