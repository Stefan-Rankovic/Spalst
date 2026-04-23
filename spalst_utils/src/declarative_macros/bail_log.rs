//! SPDX-License-Identifier: GPL-3.0-only

/// Log the given message using `tracing::error!()`, and then `bail!()` with the same message.
#[macro_export]
macro_rules! bail_log {
    ($($arg:tt)*) => {{
        tracing::error!($($arg)*);
        color_eyre::eyre::bail!($($arg)*);
    }};
}
