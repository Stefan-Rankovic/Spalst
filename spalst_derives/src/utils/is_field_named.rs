//! SPDX-License-Identifier: GPL-3.0-only

use syn::{Field, Ident};

pub fn is_field_named(
    field: &Field,
    name: &str,
) -> bool {
    field
        .ident
        .as_ref()
        .is_some_and(|ident: &Ident| ident == name)
}
