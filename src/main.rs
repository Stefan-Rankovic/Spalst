use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handler
    color_eyre::install()?;
    // Actually run the program (and return the Result)
    spalst::run().await
}
