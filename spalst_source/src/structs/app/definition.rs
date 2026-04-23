//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{ArgsParser, DisplayManager, Logger};
use color_eyre::eyre::Result;
use core::{
    sync::atomic::{AtomicU8, Ordering},
    time::Duration,
};
use crossterm::event::{Event, EventStream};
use futures::StreamExt as _;
use ratatui::{Terminal, prelude::CrosstermBackend};
use spalst_updater::structs::ProgramUpdater;
use std::{io::Stdout, path::Path, sync::Arc};
use tokio::{
    sync::mpsc,
    time::{Instant, MissedTickBehavior, interval},
};
use tracing::instrument;

/// The main struct. Everything happens inside of this.
///
/// Its state is not saved to a file.
#[derive(Debug)]
pub struct App {
    /// Arguments passed to the app.
    pub args: ArgsParser,
    /// The `Logger`.
    pub logger: Logger,

    /// The path to the parent of the executable (and all other top-level paths).
    pub parent_path: Arc<Path>,

    /// The terminal.
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    /// Manages the screen.
    ///
    /// Check `DisplayManager` documentation for more info.
    pub display: DisplayManager,

    /// A listener for `Event`s.
    pub event_listener: mpsc::Receiver<Event>,
    /// Ticks executed last second.
    pub tps: Arc<AtomicU8>,

    pub updater: Option<ProgramUpdater>,
}

impl App {
    /// How many time each tick takes.
    ///
    /// For example, a value of "1 millisecond" would mean that there are 1000 ticks every second.
    ///
    /// todo: tweak the current value, maybe
    pub const TICK_DURATION: Duration = Duration::from_millis(100);

    /// Creates a new event listener.
    #[instrument]
    pub fn event_listener() -> mpsc::Receiver<Event> {
        let (tx, rx): (mpsc::Sender<_>, mpsc::Receiver<_>) = mpsc::channel(100);

        // Asynchronous listener for events.
        // Dropping the JoinHandle<()> is fine because awaiting the Future is unnecessary.
        drop(tokio::spawn(async move {
            // Create a reader
            let mut reader: EventStream = EventStream::new();
            // If it's None then that means the stream ended
            while let Some(Ok(event)) = reader.next().await {
                // If the sending fails with an error, that means the receiever stopped listening,
                // so we can safely break the loop
                if tx.send(event).await.is_err() {
                    break;
                }
            }
        }));

        rx
    }

    #[instrument(skip(self))]
    pub async fn run(&mut self) -> Result<()> {
        let mut ticker = interval(Self::TICK_DURATION);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
        let mut second_timer: Instant = Instant::now();
        let mut tick_count: u8 = 0;

        // todo: implement exit condition
        loop {
            let _: Instant = ticker.tick().await;
            tick_count += 1;
            if second_timer.elapsed() > Duration::from_secs(1) {
                self.tps.store(tick_count, Ordering::Relaxed);
                tick_count = 0;
                second_timer = Instant::now();
            }
            self.tick().await?;
        }

        // Ok.
        // Ok(())
    }
}
