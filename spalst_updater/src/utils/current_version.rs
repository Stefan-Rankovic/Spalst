//! SPDX-License-Identifier: GPL-3.0-only

use semver::Version;

/// Gets the current version.
///
/// # Panics
/// If parsing `CARGO_PKG_VERSION` fails.
#[expect(
    clippy::expect_used,
    reason = "Unreachable unless I mess up with version numbers in Cargo.toml."
)]
#[must_use]
pub fn current_version() -> Version {
    env!("CARGO_PKG_VERSION")
        .parse()
        .expect("Failed to parse CARGO_PKG_VERSION.")
}
