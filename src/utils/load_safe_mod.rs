/// SPDX-License-Identifier: GPL-3.0-only
use crate::{traits::Saveable, utils::convert_path};
use color_eyre::eyre::{Result, bail};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn load_safe<T>(path: &Path) -> Result<T>
where
    for<'a> T: Default + Deserialize<'a> + Saveable,
{
    // Format the path
    let path: PathBuf = convert_path(path)?;
    // If the path doesn't exist, create a T instance and save it (so the path is created)
    if !path.try_exists()? {
        T::default().save(&path)?;
    };
    // Contents of the file
    let contents: String = fs::read_to_string(path)?;
    // The actual data
    let data: T = ron::from_str(&contents)?;
    // Ok.
    Ok(data)
}
