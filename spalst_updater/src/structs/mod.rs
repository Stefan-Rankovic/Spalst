//! SPDX-License-Identifier: GPL-3.0-only

mod releases;
mod safe_updater;
mod safety_level;
mod unsafe_updater;
mod version_safety;

pub(crate) use releases::Releases;
pub(crate) use safe_updater::SafeUpdater;
pub use safe_updater::SafeUpdater as SafeProgramUpdater;
pub(crate) use safety_level::SafetyLevel;
pub(crate) use unsafe_updater::UnsafeUpdater;
pub use unsafe_updater::UnsafeUpdater as UnsafeProgramUpdater;
pub(crate) use version_safety::VersionSafety;
