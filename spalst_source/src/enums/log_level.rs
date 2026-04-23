//! SPDX-License-Identifier: GPL-3.0-only

use clap::ValueEnum;
use color_eyre::eyre::{OptionExt as _, Report, Result};
use tracing::{Level, level_filters::LevelFilter};

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl TryFrom<LogLevel> for Level {
    type Error = Report;

    fn try_from(value: LogLevel) -> Result<Self> {
        // Ok.
        LevelFilter::from(value)
            .into_level()
            .ok_or_eyre("No tracing::Level equivalent of LogLevel::Off.")
    }
}

impl From<Level> for LogLevel {
    fn from(value: Level) -> Self {
        LevelFilter::from(value).into()
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => Self::OFF,
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}

impl From<LevelFilter> for LogLevel {
    fn from(value: LevelFilter) -> Self {
        match value {
            LevelFilter::OFF => Self::Off,
            LevelFilter::ERROR => Self::Error,
            LevelFilter::WARN => Self::Warn,
            LevelFilter::INFO => Self::Info,
            LevelFilter::DEBUG => Self::Debug,
            LevelFilter::TRACE => Self::Trace,
        }
    }
}
