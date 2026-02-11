//! SPDX-License-Identifier: GPL-3.0-only

pub trait EnumAsStr {
    fn as_str_debug(&self) -> &'static str;
    fn as_str_user(&self) -> &'static str;
}
