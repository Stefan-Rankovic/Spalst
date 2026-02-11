//! SPDX-License-Identifier: GPL-3.0-only

use clap::ValueEnum;
use log::LevelFilter;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum LevelFilterWrapper {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LevelFilterWrapper> for LevelFilter {
    fn from(wrapper: LevelFilterWrapper) -> Self {
        match wrapper {
            LevelFilterWrapper::Off => Self::Off,
            LevelFilterWrapper::Error => Self::Error,
            LevelFilterWrapper::Warn => Self::Warn,
            LevelFilterWrapper::Info => Self::Info,
            LevelFilterWrapper::Debug => Self::Debug,
            LevelFilterWrapper::Trace => Self::Trace,
        }
    }
}

impl From<LevelFilter> for LevelFilterWrapper {
    fn from(wrapper: LevelFilter) -> Self {
        match wrapper {
            LevelFilter::Off => Self::Off,
            LevelFilter::Error => Self::Error,
            LevelFilter::Warn => Self::Warn,
            LevelFilter::Info => Self::Info,
            LevelFilter::Debug => Self::Debug,
            LevelFilter::Trace => Self::Trace,
        }
    }
}
