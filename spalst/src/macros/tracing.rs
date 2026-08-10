//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

/// A wrapper over the `error` macro from `tracing`.
///
/// Exists because manually adding `#[cfg(feature = "logging")]` everywhere is annoying.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        #[cfg(feature = "logging")]
        tracing::error!($($arg)*);
    }};
}

/// A wrapper over the `warn` macro from `tracing`.
///
/// Exists because manually adding `#[cfg(feature = "logging")]` everywhere is annoying.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        #[cfg(feature = "logging")]
        tracing::warn!($($arg)*);
    }};
}

/// A wrapper over the `info` macro from `tracing`.
///
/// Exists because manually adding `#[cfg(feature = "logging")]` everywhere is annoying.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        #[cfg(feature = "logging")]
        tracing::info!($($arg)*);
    }};
}

/// A wrapper over the `debug` macro from `tracing`.
///
/// Exists because manually adding `#[cfg(feature = "logging")]` everywhere is annoying.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        #[cfg(feature = "logging")]
        tracing::debug!($($arg)*);
    }};
}

/// A wrapper over the `trace` macro from `tracing`.
///
/// Exists because manually adding `#[cfg(feature = "logging")]` everywhere is annoying.
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        #[cfg(feature = "logging")]
        tracing::trace!($($arg)*);
    }};
}
