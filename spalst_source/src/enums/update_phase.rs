//! SPDX-License-Identifier: GPL-3.0-only

use color_eyre::eyre::{Result, bail};

#[derive(Debug, Default)]
pub enum UpdatePhase {
    #[default]
    Nothing,
    CheckedForUpdates,
    Done,
}

impl UpdatePhase {
    /// Switches to the next phase.
    ///
    /// # Errors
    /// If `self` is `Self::Done`.
    pub fn next(&mut self) -> Result<()> {
        *self = match *self {
            Self::Nothing => Self::CheckedForUpdates,
            Self::CheckedForUpdates => Self::Done,
            Self::Done => bail!("UpdatePhase::Done has no next phase."),
        };
        // Ok.
        Ok(())
    }

    /// Switches to the next phase.
    ///
    /// # Errors
    /// If `self` is `Self::Done`.
    pub fn with_next(mut self) -> Result<Self> {
        self.next()?;
        // Ok.
        Ok(self)
    }
}
