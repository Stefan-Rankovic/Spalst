/// SPDX-License-Identifier: GPL-3.0-only
pub struct EnsureTerminalRestore;
impl Drop for EnsureTerminalRestore {
    fn drop(&mut self) {
        ratatui::restore()
    }
}
