//! SPDX-License-Identifier: GPL-3.0-only
use crate::utils::convert_path;
use color_eyre::eyre::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};

pub trait Saveable: Serialize {
    fn save(&self, path: &Path) -> Result<()> {
        // Format the path
        let path: PathBuf = convert_path(path)?;
        let ron_string: String =
            ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        std::fs::write(path, ron_string)?;
        // Ok.
        Ok(())
    }
}
