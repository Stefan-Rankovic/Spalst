//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::SafeUpdater, utils::update_to_release};
use color_eyre::eyre::Result;
use octocrab::models::repos::Release;

impl SafeUpdater {
    pub async fn update_to_release(
        &self,
        target_release: &Release,
    ) -> Result<()> {
        update_to_release(target_release).await
    }
}
