//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

#[cfg(feature = "logging")]
use crate::structs::Logger;
use crate::{
    enums::{Request, ScreenManagerRequest},
    structs::{ArgsParser, ScreenId, ScreenManager},
};
use clap::Parser as _;
use color_eyre::{Report, eyre::Result};
use egui::{Context, ViewportCommand};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::instrument;

/// todo: move to directory
#[derive(Debug)]
pub struct AppState {
    /// The error encountered while handling requests.
    handle_error: Option<Report>,
    /// The error encountered while rendering requests.
    ///
    /// Located here because `App` is dropped when `eframe::run_native` is called (inside `main()`).
    /// So extracting the error from `App` is impossible.
    render_error: Option<Report>,

    /// CLI arguments passed to the program.
    args: ArgsParser,
    /// The logger.
    #[cfg(feature = "logging")]
    pub logger: Option<Logger>,
    /// The `Screen` manager.
    pub screen_manager: ScreenManager,
}

impl AppState {
    /// Set the handle error to `None` and return the previous value.
    pub const fn take_handle_error(&mut self) -> Option<Report> {
        self.handle_error.take()
    }

    /// Set the handle error to `None` and return the previous value.
    pub const fn take_render_error(&mut self) -> Option<Report> {
        self.render_error.take()
    }

    /// Sets the handle error to the passed `Report`.
    ///
    /// # Panics
    /// If the handle error is already set.
    fn set_handle_error(
        &mut self,
        error: Report,
    ) {
        assert!(self.handle_error.is_none(), "Handle error is already set.");
        self.handle_error = Some(error);
    }

    /// Sets the render error to the passed `Report`.
    ///
    /// # Panics
    /// If the render error is already set.
    pub fn set_render_error(
        &mut self,
        error: Report,
    ) {
        assert!(self.render_error.is_none(), "Render error is already set.");
        self.render_error = Some(error);
    }

    /// Tries to get a new `App` instance.
    ///
    /// # Errors
    /// If initializing the `Logger` fails (`Logger::try_init_new` function). Exclusive to the
    /// `logging` feature.
    pub async fn try_new() -> Result<Self> {
        let args: ArgsParser = ArgsParser::parse();
        #[cfg(feature = "logging")]
        let logger: Option<Logger> = args.log.then_some(Logger::try_init_new(&args).await?);
        let screen_manager: ScreenManager = ScreenManager::new();

        Ok(Self {
            handle_error: None,
            render_error: None,
            args,
            #[cfg(feature = "logging")]
            logger,
            screen_manager,
        })
    }

    /// Listens for requests and processes them as they come.
    ///
    /// If handling a request fails, the channel is closed, the display is shut off, and the handle
    /// error is set to whatever the error was.
    ///
    /// If the channel is closed from the other side, the handling of requests is immediately
    /// stopped.
    pub async fn process_requests(
        state: Arc<Mutex<Self>>,
        mut rx: UnboundedReceiver<Request>,
        ctx: Context,
    ) {
        while let Some(request) = rx.recv().await {
            if rx.is_closed() {
                return;
            }

            let result: Result<()> = Self::handle_request(Arc::clone(&state), request).await;
            if let Err(report) = result {
                rx.close();
                ctx.send_viewport_cmd(ViewportCommand::Close);
                state.lock().unwrap().set_handle_error(report);
            }
        }
    }

    /// Handle an individual request.
    #[instrument(skip(state))]
    async fn handle_request(
        state: Arc<Mutex<Self>>,
        request: Request,
    ) -> Result<()> {
        match request {
            Request::Quit => todo!(),
            Request::ScreenManager(sm_request) => {
                let screen_manager: &mut ScreenManager = &mut state.lock().unwrap().screen_manager;
                match sm_request {
                    ScreenManagerRequest::AddScreen { parent_id, screen } => screen_manager
                        .add_screen(parent_id, screen)
                        .map(|_: ScreenId| ()),
                    ScreenManagerRequest::SelectScreen(screen_id) => screen_manager.select_screen(screen_id),
                    ScreenManagerRequest::AddAndSelectScreen { parent_id, screen } => screen_manager
                        .add_select_screen(parent_id, screen)
                        .map(|_: ScreenId| ()),
                    ScreenManagerRequest::Back => screen_manager.back().map(|_: ScreenId| ()),
                }
            }
        }
    }
}
