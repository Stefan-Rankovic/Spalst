//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{enums::Request, structs::AppState};
use color_eyre::Result;
use egui::{Context, ViewportCommand};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedReceiver;

impl AppState {
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
}
