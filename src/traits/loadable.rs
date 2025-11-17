/// SPDX-License-Identifier: GPL-3.0-only
use crate::utils::convert_path;
use color_eyre::eyre::{Context, Result, bail};
use serde::Deserialize;
use std::path::{Path, PathBuf};

pub trait Loadable: for<'a> Deserialize<'a> {
    fn load(path: &Path) -> impl Future<Output = Result<Self>> + Send {
        async move {
            // Format the path
            let path: &Path = &convert_path(path)?;
            // If the path doesn't exist, return an error.
            if !path.try_exists()? {
                bail!(
                    "Tried loading object from path {} which doesn't exist.",
                    path.display()
                );
            };
            // Contents of the file
            let contents: String = tokio::fs::read_to_string(path)
                .await
                .wrap_err_with(|| format!("Tried reading from file {}.", path.display()))?;
            // The actual data
            let data: Self = ron::from_str(&contents)
                .wrap_err_with(|| format!("Tried parsing from {}.", path.display()))?;
            // Ok.
            Ok(data)
        }
    }
}
