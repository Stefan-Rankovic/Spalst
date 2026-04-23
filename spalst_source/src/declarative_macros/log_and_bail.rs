//! SPDX-License-Identifier: GPL-3.0-only

/// Log the given message using `tracing::error!()`, and then `bail!()` with the same message.
#[macro_export]
macro_rules! log_and_bail {
    ($($arg:tt)*) => {{
        tracing::error!($($arg)*);
        color_eyre::eyre::bail!($($arg)*);
    }};
}
