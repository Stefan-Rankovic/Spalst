//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

/// Log the given message using `tracing::error!()`, and then `bail!()` with the same message.
///
/// Safe to use without the `logging` feature, because `tracing::error!()` (and the other `tracing`
/// macros) do nothing when a logging subscriber wasn't initialized.
#[macro_export]
macro_rules! bail_log {
    ($($arg:tt)*) => {{
        tracing::error!($($arg)*);
        color_eyre::eyre::bail!($($arg)*);
    }};
}
