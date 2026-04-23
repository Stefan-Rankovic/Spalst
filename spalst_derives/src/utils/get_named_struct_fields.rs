//! SPDX-License-Identifier: GPL-3.0-only

use syn::{Data, DeriveInput, Field, Fields, punctuated::Punctuated, token::Comma};

/// Get all the struct fields of a given struct.
///
/// # Errors
/// If the passed data isn't a struct.
/// If the passed struct doesn't have named fields.
pub fn get_named_struct_fields(input: &DeriveInput) -> Result<&Punctuated<Field, Comma>, syn::Error> {
    let Data::Struct(ref data_struct) = input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "This derive macro only works for structs.",
        ));
    };

    let Fields::Named(ref fields_named) = data_struct.fields else {
        return Err(syn::Error::new_spanned(
            input,
            "This derive macro only works for structs with named fields",
        ));
    };

    Ok(&fields_named.named)
}
