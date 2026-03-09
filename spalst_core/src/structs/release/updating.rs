//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::{Asset, Release};
use bytes::Bytes;
use color_eyre::eyre::{Context as _, OptionExt as _, Result};
use log::info;
use reqwest::{Client, Response};
use std::{env, path::PathBuf};

impl Release {
    /// Find the executable asset for the user's OS.
    fn find_executable_asset(&self) -> Option<&Asset> {
        // What to search for
        let target: &str = if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            return None;
        };
        // Search for the target
        self.assets
            .iter()
            .find(|asset: &&Asset| -> bool { asset.name.to_lowercase().contains(target) })
    }

    async fn download_executable(&self, asset: &Asset) -> Result<Bytes> {
        let client: Client = Client::new();
        let response: Response = client
            .get(&asset.browser_download_url)
            .header("User-Agent", "spalst_updater")
            .send()
            .await
            .wrap_err_with(|| "Failed to download new version.")?;
        let bytes: Bytes = response
            .bytes()
            .await
            .wrap_err_with(|| "Failed to read download response.")?;

        // Ok.
        Ok(bytes)
    }

    /// Update to this release
    pub async fn update_to(&self) -> Result<()> {
        // Get the current executable path
        let current_exe: PathBuf = env::current_exe()?;
        // Find the correct asset for the user's OS
        let asset: &Asset = self
            .find_executable_asset()
            .ok_or_eyre("No asset found for your OS.")?;
        // Inform the user and also log
        info!(
            "Downloading version {} from {}",
            self.version(),
            asset.browser_download_url
        );
        eprintln!("Downloading version {}...", self.version());
        // Download the update
        let bytes: Bytes = self.download_executable(asset).await?;
        // Log
        info!("Download complete.");
        eprintln!("Download complete.");
        // Actually update
        #[cfg(target_family = "unix")]
        self.unix_update(&current_exe, bytes).await?;
        #[cfg(target_family = "windows")]
        self.windows_update(&current_exe, bytes).await?;
        #[cfg(not(any(target_family = "windows", target_family = "unix")))]
        color_eyre::eyre::bail!(
            "Automatic updates are not supported on your operating system. Please update manually."
        );
        // Ok.
        Ok(())
    }
}
