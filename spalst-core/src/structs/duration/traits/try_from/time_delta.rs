//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Duration;
use chrono::TimeDelta;
use color_eyre::eyre::{Report, Result};

impl TryFrom<TimeDelta> for Duration {
    type Error = Report;

    fn try_from(td: TimeDelta) -> Result<Self> {
        Ok(td.to_std()?.into())
    }
}
