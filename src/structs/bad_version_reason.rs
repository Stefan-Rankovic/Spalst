use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct BadVersionReason(String);

impl Display for BadVersionReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
