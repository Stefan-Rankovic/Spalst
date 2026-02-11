//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Release;
use log::info;

impl Release {
    pub fn display_notes(&self) {
        // Log.
        info!(
            "Displaying release notes for version {}, which is {}.",
            self.version(),
            self.safety().reason().map_or_else(
                || "not marked as unsafe".to_string(),
                |reason| format!("marked as unsafe with the attached reason \"{reason}\"")
            )
        );
        let bad_version_note: String = self.safety().reason()
        .map(|reason| format!(
            " (note: you will not be able to update to this version as it marked unsafe. The attached reason is \"{reason}\")",
        ))
        .unwrap_or_default();
        // Display notes, if any.
        if let Some(ref notes) = self.body {
            eprintln!(
                "Release notes for release {}{}:",
                self.version(),
                bad_version_note
            );
            termimad::print_text(notes);
        } else {
            eprintln!(
                "Release {} has no release notes{}.",
                self.version(),
                bad_version_note
            );
        }
    }
}
