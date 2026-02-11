//! SPDX-License-Identifier: GPL-3.0-only

use crate::structs::Duration;
use tokio::time::Duration as DurationTokio;

impl From<DurationTokio> for Duration {
    fn from(d: DurationTokio) -> Self {
        let nanoseconds: u64 = d.subsec_nanos() as u64;

        let total_secs: u64 = d.as_secs();

        let years: u64 = total_secs / (365 * 24 * 3600);
        let remaining: u64 = total_secs % (365 * 24 * 3600);

        let months = remaining / (30 * 24 * 3600);
        let remaining = remaining % (30 * 24 * 3600);

        let days: u64 = remaining / (24 * 3600);
        let remaining: u64 = remaining % (24 * 3600);

        let hours: u64 = remaining / 3600;
        let remaining: u64 = remaining % 3600;

        let minutes: u64 = remaining / 60;

        let seconds: u64 = remaining % 60;

        Self::new(nanoseconds, seconds, minutes, hours, days, months, years)
    }
}
