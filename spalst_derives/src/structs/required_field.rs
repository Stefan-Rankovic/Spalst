//! SPDX-License-Identifier: GPL-3.0-only

/// A required field on a struct.
///
/// Must have this field in order to implement a certain trait.
#[derive(Debug)]
pub struct RequiredField {
    pub name: &'static str,
    /// The exact type of the field.
    ///
    /// It's too complicated (probably impossible too, I don't know) to actually match the correct
    /// type. Matching the name used is enough.
    pub ty: &'static str,
}

impl RequiredField {
    pub const fn new(
        name: &'static str,
        ty: &'static str,
    ) -> Self {
        Self { name, ty }
    }
}
