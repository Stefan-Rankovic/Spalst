/// SPDX-License-Identifier: GPL-3.0-only
use crate::structs::Asset;
use bytes::Bytes;
use color_eyre::eyre::{Context, OptionExt, Report, Result};
use reqwest::{Client, Response};
use semver::Version;
use serde::Deserialize;
use std::{env, path::PathBuf};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub body: Option<String>,
    assets: Vec<Asset>,
}

impl TryFrom<Release> for Version {
    type Error = Report;
    fn try_from(release: Release) -> Result<Self> {
        Ok(release.tag_name.parse()?)
    }
}
impl TryFrom<&Release> for Version {
    type Error = Report;
    fn try_from(release: &Release) -> Result<Self> {
        Ok(release.tag_name.parse()?)
    }
}

impl Release {
    /// Gets the version for self.
    pub fn try_as_version(&self) -> Result<Version> {
        self.try_into()
    }

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
            self.try_as_version()?,
            asset.browser_download_url
        );
        eprintln!("Downloading version {}...", self.try_as_version()?);
        // Download the update
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
            .wrap_err_with(|| "Failed to read download response")?;
        // Log
        info!("Download complete.");
        println!("Download complete.");
        // Actually update
        #[cfg(target_family = "unix")]
        self.unix_update(current_exe, bytes).await?;
        #[cfg(target_family = "windows")]
        Self::windows_update(current_exe, bytes).await?;
        #[cfg(not(any(target_family = "windows", target_family = "unix")))]
        bail!(
            "Automatic updates are not supported on your operating system. Please update manually."
        );
        // Ok.
        Ok(())
    }

    #[cfg(target_family = "windows")]
    async fn windows_update(current_exe: PathBuf, bytes: Bytes) -> Result<()> {
        // Save the file with a "-new" suffix.
        let current_name = current_exe.file_stem().unwrap().to_string_lossy();
        let new_exe_path = current_exe.with_file_name(format!("{}-new.exe", current_name));
        let mut new_file: File = File::create(&new_exe_path).await.wrap_err_with(|| {
            format!(
                "Failed to create executable file at location {}.",
                new_exe_path.display()
            )
        })?;
        new_file
            .write_all(&bytes)
            .await
            .wrap_err_with(|| format!("Failed to write to location {}.", new_exe_path.display()))?;
        drop(new_file);
        info!("Saved new executable at {}", new_exe_path.display());
        // Print manual update instructions.
        eprintln!("Successfully downloaded!");
        eprintln!("Sadly, due to a Windows limitation, the installation could not be completed.");
        eprintln!("Manual installation is required.");
        eprintln!("Steps:");
        eprintln!(
            "    1. Open the game installation directory (the folder where the game executable is located). Among other things, you should see the files \"spalst.exe\" and \"spalst-new.exe\"."
        );
        eprintln!("    2. Delete the \"spalst.exe\" file.");
        eprintln!("    3. Rename the \"spalst-new.exe\" file to \"spalst.exe\".");
        eprintln!("    4. Start the \"spalst.exe\" file.");
        eprintln!("    5. Start playing again!");
        // Exit the program so the user can safely delete the executable file.
        std::process::exit(0);
    }

    #[cfg(target_family = "unix")]
    async fn unix_update(&self, current_exe: PathBuf, bytes: Bytes) -> Result<()> {
        use tokio::process::Command;

        println!("Installing...");
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
        println!("Update complete.");
        println!("Restarting...");
        info!(
            "Successfully updated to version {}.",
            self.try_as_version()?
        );
        // Restart
        Command::new(&current_exe)
            .spawn()
            .wrap_err_with(|| "Failed to start program.")?;
        // Exit the old process
        std::process::exit(0);
    }
}
