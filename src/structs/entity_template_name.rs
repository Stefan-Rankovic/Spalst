/// SPDX-License-Identifier: GPL-3.0-only
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq)]
pub struct EntityTemplateName(pub String);
impl Display for EntityTemplateName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
