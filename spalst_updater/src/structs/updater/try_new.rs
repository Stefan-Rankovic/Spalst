//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::Phase,
    structs::{Releases, Updater},
};
use color_eyre::eyre::Result;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::{sync::Mutex, task::JoinHandle};
use tracing::instrument;

impl Updater {
    /// Tries to get a new `Self` instance.
    ///
    /// Will run a check for whether the current release is unsafe.
    /// If so, it will automatically start updating in another thread and return the `JoinHandle` of
    /// that task.
    /// This is not an avoidable step.
    ///
    /// # Errors
    /// If fetching `Releases` fails.
    #[instrument]
    pub async fn try_new() -> Result<(Arc<Self>, Option<JoinHandle<Result<()>>>)> {
        let phase: Arc<Mutex<Phase>> = Arc::new(Mutex::new(Phase::default()));
        let releases: Arc<Mutex<Releases>> = Arc::new(Mutex::new(Releases::fetch().await?));
        let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let this: Arc<Self> = Arc::new(Self {
            phase,
            releases,
            running,
        });
        let this_clone: Arc<Self> = Arc::clone(&this);

        let join_handle: Option<JoinHandle<Result<()>>> = if this
            .releases
            .lock()
            .await
            .safety_level_of_current()
            .is_safe()
        {
            None
        } else {
            Some(tokio::spawn(async move {
                // this_clone.phase.lock().await.update_from_unsafe();
                this_clone.update_to_safe().await
            }))
        };
        Ok((this, join_handle))
    }
}
