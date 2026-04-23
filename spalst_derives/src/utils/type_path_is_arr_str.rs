//! SPDX-License-Identifier: GPL-3.0-only

use syn::{PathSegment, TypePath};

pub fn type_path_is_arr_str(
    type_path: &TypePath,
    arr_str: &[&str],
) -> bool {
    let segments = &type_path.path.segments;

    segments.len() == arr_str.len()
        && segments
            .iter()
            .zip(arr_str)
            .all(|(segment, expected): (&PathSegment, &&str)| segment.ident == expected)
}
