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
            LevelFilterWrapper::Off => LevelFilter::Off,
            LevelFilterWrapper::Error => LevelFilter::Error,
            LevelFilterWrapper::Warn => LevelFilter::Warn,
            LevelFilterWrapper::Info => LevelFilter::Info,
            LevelFilterWrapper::Debug => LevelFilter::Debug,
            LevelFilterWrapper::Trace => LevelFilter::Trace,
        }
    }
}
