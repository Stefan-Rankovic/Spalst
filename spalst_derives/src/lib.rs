//! SPDX-License-Identifier: GPL-3.0-only

mod derive_macros;
mod structs;
mod utils;

use crate::derive_macros::{derive_blockable_impl, derive_styled_impl};
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[expect(missing_docs, reason = "Self-explanatory.")]
#[proc_macro_derive(Styled)]
pub fn derive_styled(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    derive_styled_impl(&input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[expect(missing_docs, reason = "Self-explanatory.")]
#[proc_macro_derive(Blockable)]
pub fn derive_blockable(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    derive_blockable_impl(&input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}
