#[derive(Debug, Eq, PartialEq)]
pub enum MergePriority {
    Self_,
    Other,
    Error,
    Unreachable,
}
