//! SPDX-License-Identifier: GPL-3.0-only

use core::time::Duration;

pub fn display_duration_ago_format(duration: Duration) -> String {
    display_duration(duration) + "ago"
}

pub fn display_duration(duration: Duration) -> String {
    let td = duration.as_secs(); // [T]otal [D]uration
    let years = td.div_euclid(365 * 24 * 3600);
    let months = (td % (365 * 24 * 3600)).div_euclid(30 * 24 * 3600);
    let days = (td % (30 * 24 * 3600)).div_euclid(24 * 3600);
    let hours = (td % (24 * 3600)).div_euclid(3600);
    let minutes = (td % 3600).div_euclid(60);
    let seconds = td % 60;

    let array = [
        (years, "year", "years"),
        (months, "month", "months"),
        (days, "day", "days"),
        (hours, "hour", "hours"),
        (minutes, "minute", "minutes"),
        (seconds, "second", "seconds"),
    ];

    let significant: Vec<String> = array
        .iter()
        .filter(|&&(val, _, _)| val > 0)
        .take(2)
        .map(|&(val, singular, plural)| {
            let unit = if val == 1 { singular } else { plural };
            format!("{val} {unit}")
        })
        .collect();

    if significant.is_empty() {
        "less than a second".to_string()
    } else if significant.len() == 1 {
        significant[0].clone()
    } else {
        format!("{} and {}", significant[0], significant[1])
    }
}
