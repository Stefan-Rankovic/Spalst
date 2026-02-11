//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Release;
use bytes::Bytes;
use color_eyre::eyre::{Context, Result};
use log::info;
use std::{path::Path, process::exit};
use tokio::{fs::File, io::AsyncWriteExt as _};

impl Release {
    pub(super) async fn windows_update(&self, current_exe: &Path, bytes: Bytes) -> Result<()> {
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
        eprintln!("Successfully downloaded version {}!", self.version());
        eprintln!("Sadly, due to a Windows limitation, the installation could not be completed.");
        eprintln!("Manual installation is required.");
        eprintln!("Steps:");
        eprintln!(
            "    1. Open the game installation directory (the folder where the game executable is located). You should see the files \"spalst.exe\" and \"spalst-new.exe\" (or 'spalst' and 'spalst-new' if you have file extensions disabled)."
        );
        eprintln!("    2. Delete the \"spalst.exe\" ('spalst') file.");
        eprintln!(
            "    3. Rename the \"spalst-new.exe\" ('spalst-new') file to \"spalst.exe\" ('spalst')."
        );
        eprintln!("    4. Run the \"spalst.exe\" ('spalst') file.");
        eprintln!("    5. Start playing again!");
        // Exit the program so the user can safely delete the executable file.
        exit(0);
    }
}
