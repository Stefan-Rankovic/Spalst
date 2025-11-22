//! SPDX-License-Identifier: GPL-3.0-only

use chrono::TimeDelta;

pub fn time_delta_format(td: TimeDelta) -> String {
    if td.as_seconds_f64() < 1.0 {
        return "less than a second ago".to_string();
    };
    // How many seconds, minutes, hours, etc. elapsed
    let seconds: i64 = td.num_seconds();
    let minutes: i64 = td.num_minutes();
    let hours: i64 = td.num_hours();
    let days: i64 = td.num_days();
    let weeks: i64 = td.num_weeks();
    let years: i64 = days / 365;

    if years > 0 {
        let remaining_days: i64 = days % 365;
        if remaining_days > 0 {
            return format!(
                "{} year{} and {} day{} ago",
                years,
                if years == 1 { "" } else { "s" },
                remaining_days,
                if remaining_days == 1 { "" } else { "s" }
            );
        };
        return format!("{} year{} ago", years, if years == 1 { "" } else { "s" });
    };

    if weeks > 0 {
        let remaining_days: i64 = days % 7;
        if remaining_days > 0 {
            return format!(
                "{} week{} and {} day{} ago",
                weeks,
                if weeks == 1 { "" } else { "s" },
                remaining_days,
                if remaining_days == 1 { "" } else { "s" }
            );
        }
        return format!("{} week{} ago", weeks, if weeks == 1 { "" } else { "s" });
    }

    if days > 0 {
        let remaining_hours: i64 = hours % 24;
        if remaining_hours > 0 {
            return format!(
                "{} day{} and {} hour{} ago",
                days,
                if days == 1 { "" } else { "s" },
                remaining_hours,
                if remaining_hours == 1 { "" } else { "s" }
            );
        }
        return format!("{} day{} ago", days, if days == 1 { "" } else { "s" });
    }

    if hours > 0 {
        let remaining_minutes: i64 = minutes % 60;
        if remaining_minutes > 0 {
            return format!(
                "{} hour{} and {} minute{} ago",
                hours,
                if hours == 1 { "" } else { "s" },
                remaining_minutes,
                if remaining_minutes == 1 { "" } else { "s" }
            );
        }
        return format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" });
    }

    if minutes > 0 {
        let remaining_seconds: i64 = seconds % 60;
        if remaining_seconds > 0 {
            return format!(
                "{} minute{} and {} second{} ago",
                minutes,
                if minutes == 1 { "" } else { "s" },
                remaining_seconds,
                if remaining_seconds == 1 { "" } else { "s" }
            );
        }
        return format!(
            "{} minute{} ago",
            minutes,
            if minutes == 1 { "" } else { "s" }
        );
    }

    format!(
        "{} second{} ago",
        seconds,
        if seconds == 1 { "" } else { "s" }
    )
}
