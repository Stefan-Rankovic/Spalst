//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::RequiredField, utils::get_named_struct_fields};
use syn::{DeriveInput, Field, Ident, Type, punctuated::Punctuated, token::Comma};

/// Ensure that a given struct has all of the required fields passed.
///
/// # Errors
/// If the struct doesn't have all required fields.
pub fn ensure_required_fields(
    input: &DeriveInput,
    required_fields: &[RequiredField],
) -> Result<(), syn::Error> {
    let fields: &Punctuated<Field, Comma> = get_named_struct_fields(input)?;

    for required_field in required_fields {
        let option_field: Option<&Field> = fields.iter().find(|field: &&Field| {
            field
                .ident
                .as_ref()
                .is_some_and(|ident: &Ident| ident == required_field.name)
        });

        match option_field {
            None => {
                return Err(syn::Error::new_spanned(
                    input,
                    format!(
                        "Missing required field `{}: {}`.",
                        required_field.name, required_field.ty
                    ),
                ));
            }
            Some(field) => {
                let expected: Type = syn::parse_str(required_field.ty)?;

                if field.ty != expected {
                    return Err(syn::Error::new_spanned(
                        input,
                        format!(
                            "Field `{}` has wrong type: expected `{}`",
                            required_field.name, required_field.ty
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}
