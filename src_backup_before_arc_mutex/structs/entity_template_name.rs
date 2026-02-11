//! SPDX-License-Identifier: GPL-3.0-only

use serde::Deserialize;
use std::{
    fmt::{self, Display, Formatter},
    ops::Deref,
};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq)]
pub struct EntityTemplateName(String);

impl Deref for EntityTemplateName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for EntityTemplateName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}
