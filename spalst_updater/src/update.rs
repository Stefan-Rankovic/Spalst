//! SPDX-License-Identifier: GPL-3.0-only

use crate::{enums::Outcome, structs::Updater};
use color_eyre::eyre::Result;
use spalst_utils::bail_log;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::instrument;

#[expect(clippy::missing_errors_doc, reason = "Too many to list.")]
#[expect(missing_docs, reason = "Self-explanatory.")]
#[instrument]
pub async fn update() -> Result<Outcome> {
    let (updater, option_join_handle): (Arc<Updater>, Option<JoinHandle<Result<()>>>) = Updater::try_new().await?.into();
    if let Some(join_handle) = option_join_handle {
        join_handle.await??;
    }

    let outcome: Outcome = updater.start_updating().await?;

    // If the program was on unsafe, and the program did not update, quit.
    //
    // This runs on the assumption that `Updater` would always update when on an unsafe release, if
    // it can. Which is the case.
    if !updater.was_safe().await && !outcome.updated() {
        bail_log!("Program is on an unsafe release. Due to some reason, the outcome ended up as {outcome:?}. In other words, the program could not update.");
    }

    // Ok.
    Ok(outcome)
}
