//! SPDX-License-Identifier: GPL-3.0-only

use crate::utils::{has_field_named, type_path_is_arr_str};
use syn::{Field, PathSegment, Type, TypePath, punctuated::Punctuated, token::Comma};

pub fn has_field(
    fields: &Punctuated<Field, Comma>,
    name: Option<&str>,
    type_path: Option<&[&str]>,
    type_wrapped_in: Option<&[&str]>,
) -> bool {
    fields.iter().any(|field: &Field| {
        let has_proper_type: bool = type_path.is_none_or(|actual_type_path: &[&str]| {
            let Type::Path(ref type_path) = field.ty else {
                return false;
            };

            let inside_type_path: &TypePath = if let Some(outside_wrapper) = type_wrapped_in {
                if !type_path_is_arr_str(type_path, outside_wrapper) {
                    return false;
                }
                type_path.path.segments.last().unwrap()
            } else {
                type_path
            };

            let segments = &inside_type_path.path.segments;

            let outside_type_path_should_be = if let Some(actual_type_wrapped_in) = type_wrapped_in {
                actual_type_wrapped_in
            } else {
                segments.iter().map(|segment| segment.ident.to_string())
            };

            segments.len() == actual_type_path.len()
                && segments
                    .iter()
                    .zip(actual_type_path)
                    .all(|(segment, expected): (&PathSegment, &&str)| segment.ident == expected)
        });

        has_proper_type
    }) && name.is_none_or(|actual_name: &str| has_field_named(actual_name, fields))
}
