//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Duration;
use std::time::Duration as DurationStd;
use tokio::time::Duration as DurationTokio;

impl From<DurationStd> for Duration {
    fn from(d: DurationStd) -> Self {
        DurationTokio::from(d).into()
    }
}
