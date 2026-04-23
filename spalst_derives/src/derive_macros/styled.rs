//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::RequiredField, utils::ensure_required_fields};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

pub fn derive_styled_impl(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    const REQUIRED_FIELDS: [RequiredField; 1] = [RequiredField::new("style", "Style")];
    let name: &Ident = &input.ident;

    ensure_required_fields(input, &REQUIRED_FIELDS)?;

    Ok(quote! {
        impl Styled for #name {
            fn get_style(&self) -> Style {
                self.style
            }

            fn set_style(&mut self, new_style: Style ) -> Style {
                core::mem::replace(&mut self.style, new_style)
            }

        }
    })
}
