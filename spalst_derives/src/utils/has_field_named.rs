//! SPDX-License-Identifier: GPL-3.0-only

use crate::utils::is_field_named;
use syn::{Field, punctuated::Punctuated, token::Comma};

pub fn has_field_named(
    name: &str,
    fields: &Punctuated<Field, Comma>,
) -> bool {
    fields
        .iter()
        .any(|field: &Field| is_field_named(field, name))
}
