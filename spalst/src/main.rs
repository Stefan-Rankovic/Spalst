//! SPDX-License-Identifier: GPL-3.0-only

use color_eyre::{Result, install};
use spalst_source::run;

#[tokio::main]
async fn main() -> Result<()> {
    install()?;
    run().await
}
