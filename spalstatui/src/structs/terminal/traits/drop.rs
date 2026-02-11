//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Terminal;

impl Drop for Terminal {
    fn drop(&mut self) {
        // Attempt to restore the cursor state
        if self.hidden_cursor
            && let Err(err) = self.show_cursor()
        {
            eprintln!("Failed to show the cursor: {err}");
        }
    }
}
