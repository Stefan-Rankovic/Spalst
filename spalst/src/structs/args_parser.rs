//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

#[cfg(feature = "logging")]
use crate::enums::LogLevel;
use clap::{ArgAction, Parser};

/// Parse command-line arguments.
#[derive(Debug, Parser)]
#[command(name = "spalst")]
pub struct ArgsParser {
    /// Whether to create a logfile for the current program instance.
    #[arg(
        long,
        default_value_t = true,
        action = ArgAction::Set,
        num_args = 0..=1,
        default_missing_value = "true",
        help = "Create a log file for the current program instance."
    )]
    #[cfg(feature = "logging")]
    pub log: bool,
    /// The maximum log level.
    #[arg(
        long,
        value_enum,
        default_value_t = default_max_log_level(),
        help = "The minimum log level. Only messages of this level of importance (or higher) will be logged."
    )]
    #[cfg(feature = "logging")]
    pub log_level: LogLevel,
    /// Whether to delete all previous logs.
    /// If the current log exists, it won't be deleted.
    #[arg(
        long,
        default_value_t = false,
        help = "All previous logs will be deleted. The log of the current program instance, if one exists, won't be deleted."
    )]
    #[cfg(feature = "logging")]
    pub rm_old_logs: bool,
    /// Whether to clean the current log after the program exits successfully.
    #[arg(
        long,
        default_value_t = !dev_profile(),
        action = ArgAction::Set,
        num_args = 0..=1,
        default_missing_value = "true",
        help = "The log of this program instance will be deleted if the program exits successfully."
    )]
    #[cfg(feature = "logging")]
    pub rm_log: bool,
}

/// The default max logging level.
///
/// Computed based on whether the program is compiling in dev or release mode.
#[cfg(feature = "logging")]
const fn default_max_log_level() -> LogLevel {
    if dev_profile() {
        LogLevel::Debug
    } else {
        LogLevel::Warn
    }
}

/// Whether the program was compiled with the `dev` profile.
#[cfg(feature = "logging")]
const fn dev_profile() -> bool {
    cfg!(debug_assertions)
}
