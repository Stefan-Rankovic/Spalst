//! SPDX-License-Identifier: GPL-3.0-only

use crate::{structs::RequiredField, utils::ensure_required_fields};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

pub fn derive_blockable_impl(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    const REQUIRED_FIELDS: [RequiredField; 1] = [RequiredField::new("block", "Option<BlockDisplay>")];
    let name: &Ident = &input.ident;

    ensure_required_fields(input, &REQUIRED_FIELDS)?;

    Ok(quote! {
        impl Blockable for #name {
            fn get_block(&self) -> Option<&BlockDisplay> {
                self.block.as_ref()
            }

            fn set_block(
                &mut self,
                new_block: BlockDisplay,
            ) -> Option<BlockDisplay> {
                self.block.replace(new_block)
            }
        }
    })
}
