//! SPDX-License-Identifier: GPL-3.0-only

use crate::enums::LogLevel;
use clap::Parser;

/// Parse command-line arguments.
#[derive(Debug, Parser)]
#[command(name = "spalst")]
#[command(about = "RPG Game", version)]
pub struct ArgsParser {
    /// Maximum log level for the current program.
    #[arg(
        long,
        value_enum,
        default_value_t = LogLevel::Warn,
        help = "Log actions inside a .log file with the maximum level passed."
    )]
    pub log_level: LogLevel,
    /// Whether to delete the previous logs.
    #[arg(
        long,
        default_value_t = false,
        help = "Delete all previous logs, leaving only the log of the current program instance."
    )]
    pub clean_previous_logs: bool,
    /// Whether to clean the current log after the program is finished.
    #[arg(
        long,
        default_value_t = false,
        help = "Delete the log of the current program instance after the program finishes."
    )]
    pub clean_log_after: bool,
}
