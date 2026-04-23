//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::Updater, utils::get_asset};
use bytes::Bytes;
use color_eyre::eyre::Result;
use octocrab::models::repos::Release;

impl Updater {
    /// Update the program to a certain `Release`.
    #[expect(clippy::missing_errors_doc, reason = "Too many to list.")]
    pub async fn update_to_release(
        &self,
        target_release: &Release,
    ) -> Result<()> {
        let asset: Bytes = get_asset(target_release).await?;

        todo!();

        // Ok.
        // Ok(())
    }
}
