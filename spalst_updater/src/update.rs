//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    enums::Outcome,
    structs::{SafeUpdater, UnsafeUpdater},
};
use color_eyre::eyre::Result;
use tracing::instrument;

#[expect(clippy::missing_errors_doc, reason = "Too many to list.")]
#[instrument]
pub async fn update() -> Result<Outcome> {
    // let updater: SafeUpdater = UnsafeUpdater::try_new().await?.ensure_safe().await?;

    // updater.start_updating().await

    todo!()
}
