use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handler
    color_eyre::install()?;
    // Actually run the program
    let result: Result<()> = spalst::run().await;
    // Restore the terminal to how it was before the program started
    ratatui::restore();

    result
}
