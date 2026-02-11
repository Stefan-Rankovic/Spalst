//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Release;
use bytes::Bytes;
use color_eyre::eyre::{Context as _, Result};
use log::info;
use std::{
    path::{Path, PathBuf},
    process::exit,
};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt as _,
};

impl Release {
    pub(super) async fn unix_update(&self, current_exe: &Path, bytes: Bytes) -> Result<()> {
        eprintln!("Installing...");
        // Save file to temporary location
        let temp_path: PathBuf = current_exe.with_file_name(".spalst_new_temp");
        let mut temp_file: File = File::create(&temp_path).await.wrap_err_with(|| {
            format!(
                "Failed to create temporary file at path {}",
                temp_path.display()
            )
        })?;
        temp_file
            .write_all(&bytes)
            .await
            .wrap_err_with(|| format!("Failed to write to path {}.", temp_path.display()))?;
        drop(temp_file);
        // Copy the current permissions and set them as the permissions for the new executable as
        // well.
        let current_permissions = fs::metadata(&current_exe).await?.permissions();
        fs::set_permissions(&temp_path, current_permissions).await?;
        // Replace the current executable
        fs::rename(&temp_path, &current_exe)
            .await
            .wrap_err_with(|| "Failed to replace executable with the downloaded one.")?;
        // Log
        eprintln!("Update complete.");
        info!("Successfully updated to version {}.", self.version());
        // Exit.
        exit(0);
    }
}
