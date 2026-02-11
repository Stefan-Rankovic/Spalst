//! SPDX-License-Identifier: GPL-3.0-only

use crate::{
    traits::{Loadable, Saveable},
    utils::convert_path,
};
use color_eyre::eyre::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};

pub trait LoadableSafe:
    Default + for<'deserialize> Deserialize<'deserialize> + Loadable + Saveable
{
    /// An alternative to `Loadable::load()`.
    ///
    /// Instead of returning an error if the path doesn't exist (like `Loadable::load()`), it will
    /// create a default instance of `Self` (with the `Default` implementation), save it to the
    /// provided path, and then return it.
    ///
    /// It will behave in the exact same way as `Loadable::load()` if the provided path exists.
    async fn load_safe(path: &Path) -> Result<Self> {
        // Format the path
        let path: PathBuf = convert_path(path)?;
        // If the path doesn't exist, create a Self instance and save it (so the path is created).
        if !path.try_exists()? {
            let default: Self = Self::default();
            default.save(&path).await?;
            // Ok.
            return Ok(default);
        }
        Self::load(&path).await
    }
}
