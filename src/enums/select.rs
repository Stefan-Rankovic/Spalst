//! SPDX-License-Identifier: GPL-3.0-only

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Select<T> {
    Previous,
    Next,
    Direct(T),
}
