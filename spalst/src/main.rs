//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

mod enums;
mod macros;
mod statics;
mod structs;
mod traits;

use crate::{
    enums::Request,
    structs::{App, AppState},
};
use color_eyre::Result;
use eframe::{CreationContext, NativeOptions};
use egui::Context;
use std::sync::{Arc, Mutex};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
#[cfg(feature = "logging")]
use tracing::instrument;

#[tokio::main]
#[cfg_attr(feature = "logging", instrument)]
async fn main() -> Result<()> {
    // Initialize `color_eyre`
    color_eyre::install()?;

    let (tx, rx): (UnboundedSender<Request>, UnboundedReceiver<Request>) = mpsc::unbounded_channel();
    let app_state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::try_new().await?));

    let app: App = App::new(Arc::clone(&app_state), tx);

    eframe::run_native(
        "Spalst",
        NativeOptions::default(),
        Box::new({
            let app_state: Arc<Mutex<AppState>> = Arc::clone(&app_state);
            move |cc: &CreationContext<'_>| {
                let ctx: Context = cc.egui_ctx.clone();
                let _: JoinHandle<()> = tokio::spawn(AppState::process_requests(app_state, rx, ctx));
                Ok(Box::new(app))
            }
        }),
    )?;

    app_state
        .lock()
        .unwrap()
        .take_handle_error()
        .map_or(Ok(()), Err)?;

    app_state
        .lock()
        .unwrap()
        .take_render_error()
        .map_or(Ok(()), Err)?;

    #[cfg(feature = "logging")]
    if let Some(logger) = app_state.lock().unwrap().logger.as_mut() {
        logger.successful_exit = true;
    }

    Ok(())
}
