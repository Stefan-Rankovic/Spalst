//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

mod paths;

pub use paths::HOME_DIR;
#[cfg(feature = "logging")]
pub use paths::LOG_FILES_DIR;
