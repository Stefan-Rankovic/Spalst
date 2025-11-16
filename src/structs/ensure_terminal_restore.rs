pub struct EnsureTerminalRestore;
impl Drop for EnsureTerminalRestore {
    fn drop(&mut self) {
        ratatui::restore()
    }
}
