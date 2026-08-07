//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::{enums::Request, structs::AppState};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;

/// The main struct of this app that holds everything.
pub struct App {
    /// The actual data.
    pub state: Arc<Mutex<AppState>>,

    /// Sends requests (`Request`) to the receiver (in `AppState`).
    ///
    /// Will become `None` once a rendering error is encountered.
    /// That will drop the channel.
    pub(super) tx: Option<UnboundedSender<Request>>,
}

impl App {
    /// Tries to get a new `App` instance.
    ///
    /// # Errors
    /// If initializing the `Logger` fails (`Logger::try_init_new` function).
    pub const fn new(
        state: Arc<Mutex<AppState>>,
        tx: UnboundedSender<Request>,
    ) -> Self {
        Self {
            state,
            tx: Some(tx),
        }
    }
}
