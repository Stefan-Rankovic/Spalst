//! SPDX-License-Identifier: GPL-3.0-only

use core::fmt::Display;
use log::warn;

pub fn epr_wrn<M: Display>(msg: M) {
    eprintln!("{msg}");
    warn!("{msg}");
}
