//! SPDX-License-Identifier: GPL-3.0-only

use core::fmt::Display;
use log::debug;

pub fn epr_dbg<M: Display>(msg: M) {
    eprintln!("{msg}");
    debug!("{msg}");
}
