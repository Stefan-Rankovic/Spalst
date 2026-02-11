//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Terminal;
use color_eyre::eyre::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use ratatui::restore;
use std::{io::stdout, panic};

/// Initializes the terminal.
///
/// # Panics
/// - If enabling raw mode fails.
/// - If entering an alternate screen fails.
#[must_use]
#[expect(
    clippy::expect_used,
    reason = "The purpose of this function is to be an expect wrapper around try_init()."
)]
pub fn init() -> Terminal {
    try_init().expect("failed to initialize terminal")
}

/// Tries to initialize the terminal.
///
/// # Errors
/// - If enabling raw mode fails.
/// - If entering an alternate screen fails.
pub fn try_init() -> Result<Terminal> {
    set_panic_hook();
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Terminal::new()
}

fn set_panic_hook() {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        restore();
        hook(info);
    }));
}
