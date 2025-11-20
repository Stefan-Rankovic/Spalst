//! SPDX-License-Identifier: GPL-3.0-only
#[derive(Debug, Eq, PartialEq)]
pub enum MergePriority {
    Self_,
    Other,
    Error,
    Unreachable,
}
