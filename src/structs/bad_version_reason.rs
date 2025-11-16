/// SPDX-License-Identifier: GPL-3.0-only
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct BadVersionReason(String);

impl Display for BadVersionReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
