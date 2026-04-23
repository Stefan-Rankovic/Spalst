//! SPDX-License-Identifier: GPL-3.0-only

use crossterm::event::Event;
use strum::EnumIs;

/// An "event" for a `MenuThing` to handle.
///
/// Because some `MenuThing`s may want to periodically check something (like a timer), there's a
/// `Tick` variant for that, to represent nothing actually happened.
#[derive(Debug, EnumIs)]
pub enum AppEvent {
    /// An `Event` happened.
    Event(Event),
    /// Nothing happened, but `MenuThing`s need to get a check-up.
    Tick,
}
