use color_eyre::eyre::Result;
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io::Stdout;

struct EnsureTerminalRestore;
impl Drop for EnsureTerminalRestore {
    fn drop(&mut self) {
        ratatui::restore()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handler
    color_eyre::install()?;
    // Ensure that the terminal is always restored to how it was before the program started
    let _restore: EnsureTerminalRestore = EnsureTerminalRestore;
    // Initialize the UI
    let terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();
    // Actually run the program (and return the Result)
    spalst::run(terminal).await
}
