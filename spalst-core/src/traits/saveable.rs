//! SPDX-License-Identifier: GPL-3.0-only

use crate::utils::convert_path;
use color_eyre::eyre::Result;
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tokio::fs;

pub trait Saveable: Serialize {
    async fn save(&self, path: &Path) -> Result<()> {
        let path: PathBuf = convert_path(path)?;
        let ron_string: String = to_string_pretty(self, PrettyConfig::default())?;

        fs::write(path, ron_string).await?;

        // Ok.
        Ok(())
    }
}
