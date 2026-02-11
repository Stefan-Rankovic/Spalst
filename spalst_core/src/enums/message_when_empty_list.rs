//! SPDX-License-Identifier: GPL-3.0-only

/// The message to display when the list is empty.
#[derive(Debug)]
pub enum MessageWhenEmptyList {
    /// No message.
    None,
    /// Default message. This may have the same behaviour as `None`.
    Default,
    /// Specified message.
    Some(&'static str),
}
