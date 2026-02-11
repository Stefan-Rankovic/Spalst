//! SPDX-License-Identifier: GPL-3.0-only

use crate::utils::convert_path;
use color_eyre::{
    Section as _,
    eyre::{Context as _, Result, bail},
};
use serde::Deserialize;
use std::path::Path;
use tokio::fs;

pub trait Loadable: for<'deserialize> Deserialize<'deserialize> {
    fn load(path: &Path) -> impl Future<Output = Result<Self>> + Send {
        async move {
            // Format the path
            let path: &Path = &convert_path(path)?;
            // If the path doesn't exist, return an error.
            if !path.try_exists()? {
                // Err.
                bail!(
                    "Tried loading object from path {} which doesn't exist.",
                    path.display()
                );
            }
            // Contents of the file.
            let contents: String = fs::read_to_string(path)
                .await
                .wrap_err_with(|| format!("Tried reading from file {}.", path.display()))
                .with_suggestion(
                    || "Maybe the file's permissions don't allow this program to access it?",
                )?;
            // The actual data.
            let data: Self = ron::from_str(&contents)
                .wrap_err_with(|| format!("Tried parsing from {}.", path.display()))
                .with_suggestion(
                    || "Maybe you edited this file manually and didn't properly format it?",
                )?;
            // Ok.
            Ok(data)
        }
    }
}
