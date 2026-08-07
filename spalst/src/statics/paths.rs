//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use crate::structs::AbsolutePathBuf;
use home::home_dir;
use std::sync::LazyLock;

/// The user's home directory.
#[expect(
    clippy::expect_used,
    reason = "Home directory is essential to the program and must be available."
)]
pub static HOME_DIR: LazyLock<AbsolutePathBuf> = LazyLock::new(|| {
    home_dir()
        .expect("Unable to determine home directory")
        .try_into()
        .expect("Home directory is a relative path")
});

/// The directory containing logfiles.
pub static LOG_FILES_DIR: LazyLock<AbsolutePathBuf> = LazyLock::new(|| HOME_DIR.join(".local/state/spalst/logs"));
