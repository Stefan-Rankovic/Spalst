//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use clap::ValueEnum;
use tracing::Level;

/// The log level.
///
/// This exists because `tracing::Level` doesn't implement `clap::ValueEnum`.
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum LogLevel {
    #[expect(clippy::missing_docs_in_private_items, reason = "Self-explanatory.")]
    Error,
    #[expect(clippy::missing_docs_in_private_items, reason = "Self-explanatory.")]
    Warn,
    #[expect(clippy::missing_docs_in_private_items, reason = "Self-explanatory.")]
    Info,
    #[expect(clippy::missing_docs_in_private_items, reason = "Self-explanatory.")]
    Debug,
    #[expect(clippy::missing_docs_in_private_items, reason = "Self-explanatory.")]
    Trace,
}

impl From<LogLevel> for Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}
