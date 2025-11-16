/// SPDX-License-Identifier: GPL-3.0-only
use crate::utils::convert_path;
use color_eyre::eyre::{Result, bail};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn load<T>(path: &Path) -> Result<T>
where
    for<'a> T: Deserialize<'a>,
{
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
    let contents: String = fs::read_to_string(path)?;
    // The actual data
    let data: T = ron::from_str(&contents)?;
    // Ok.
    Ok(data)
}
