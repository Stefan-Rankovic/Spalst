//! SPDX-License-Identifier: GPL-3.0-only
use crate::{
    traits::{Loadable, Saveable},
    utils::convert_path,
};
use color_eyre::eyre::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};

pub trait LoadableSafe: Default + for<'a> Deserialize<'a> + Loadable + Saveable {
    fn load_safe(path: &Path) -> impl Future<Output = Result<Self>> + Send {
        async move {
            // Format the path
            let path: PathBuf = convert_path(path)?;
            // If the path doesn't exist, create a Self instance and save it (so the path is created)
            if !path.try_exists()? {
                Self::default().save(&path)?;
            };
            Self::load(&path).await
        }
    }
}
