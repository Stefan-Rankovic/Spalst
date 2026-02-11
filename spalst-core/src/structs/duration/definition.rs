//! SPDX-License-Identifier: GPL-3.0-only

use color_eyre::eyre::{Result, bail};
use field_names::FieldNames;
use getset::Getters;
use serde::{Deserialize, Serialize};

// todo: maybe just make 2 util functions that take std::Duration or tokio::Duration and display them instead of... an entire struct
/// A duration.
///
/// The nanoseconds currently have no use, which is subject to change. However, such changes are
/// coming basically never and shouldn't be expected. Still a todo, I guess.
///
/// Months are assumed to be 30 days.
#[derive(Clone, Copy, Debug, Deserialize, Eq, FieldNames, Getters, PartialEq, Serialize)]
#[getset(get = "pub")]
#[deprecated = "use std::time::Duration instead"]
pub struct Duration {
    nanoseconds: u64,
    seconds: u64,
    minutes: u64,
    hours: u64,
    days: u64,
    months: u64,
    years: u64,
}

impl Duration {
    /// Fields.
    ///
    /// Because for some reason Self::FIELDS is private (who made that API???) and this method
    /// fixes that.
    pub const fn fields() -> &'static [&'static str] {
        &Self::FIELDS
    }

    /// Gets the value of the field based on the &str passed.
    ///
    /// # Errors
    /// Only fails if the &str passed isn't a valid field.
    pub fn get_field(&self, name: &str) -> Result<&u64> {
        Ok(match name {
            "nanoseconds" => self.nanoseconds(),
            "seconds" => self.seconds(),
            "minutes" => self.minutes(),
            "hours" => self.hours(),
            "days" => self.days(),
            "months" => self.months(),
            "years" => self.years(),
            _ => bail!(
                "Passed name {} is not a valid field for struct Duration.",
                name
            ),
        })
    }

    pub const fn nanoseconds_total(&self) -> u64 {
        self.nanoseconds + self.secs_total() * 1_000_000_000
    }

    /// Returns the total duration as seconds.
    ///
    /// Ignores nanoseconds.
    pub const fn secs_total(&self) -> u64 {
        self.seconds
            + self.minutes * 60
            + self.hours * 3600
            + self.days * 24 * 3600
            + self.months * 30 * 24 * 3600
            + self.years * 365 * 24 * 3600
    }

    pub const fn new(
        nanoseconds: u64,
        seconds: u64,
        minutes: u64,
        hours: u64,
        days: u64,
        months: u64,
        years: u64,
    ) -> Self {
        Self {
            nanoseconds,
            seconds,
            minutes,
            hours,
            days,
            months,
            years,
        }
    }

    pub const fn add_seconds(&mut self, seconds: u64) {
        self.seconds += seconds;
        if self.seconds >= 60 {
            self.add_minutes(self.seconds / 60);
            self.seconds %= 60;
        };
    }
    pub const fn add_minutes(&mut self, minutes: u64) {
        self.minutes += minutes;
        if self.minutes >= 60 {
            self.add_hours(self.minutes / 60);
            self.minutes %= 60;
        };
    }
    pub const fn add_hours(&mut self, hours: u64) {
        self.hours += hours;
        if self.hours >= 24 {
            self.add_days(self.hours / 24);
            self.hours %= 24;
        };
    }
    pub const fn add_days(&mut self, days: u64) {
        self.days += days;
        if self.days >= 30 {
            self.add_months(self.days / 30);
            self.days %= 30;
        };
    }
    pub const fn add_months(&mut self, months: u64) {
        self.months += months;
        if self.months >= 12 {
            self.add_years(self.months / 12);
            self.months %= 12;
        };
    }
    pub const fn add_years(&mut self, years: u64) {
        self.years += years;
    }
}
