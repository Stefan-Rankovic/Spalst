/// SPDX-License-Identifier: GPL-3.0-only
use color_eyre::eyre::{Result, bail};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::utils::convert_path;

pub trait Loadable: for<'a> Deserialize<'a> {
    fn load(path: &Path) -> Result<Self> {
        // Format the path
        let path: PathBuf = convert_path(path)?;
        // If the path doesn't exist, return an error.
        if !path.try_exists()? {
            bail!(
                "Tried loading object from path {} which doesn't exist.",
                path.display()
            );
        };
        // Contents of the file
        let contents: String = std::fs::read_to_string(path)?;
        // The actual data
        let data: Self = ron::from_str(&contents)?;
        // Ok.
        Ok(data)
    }
}
