//! SPDX-License-Identifier: GPL-3.0-only
use color_eyre::{
    Section,
    eyre::{Result, bail},
};
use std::path::{Path, PathBuf};

/// Converts a given Path to the actual save location. It also checks things like is the path
/// something not a file, and does the user have permissions to read that directory and check if the
/// file exists.
///
/// # Examples
/// ```
/// use spalst::utils::convert_path;
/// use std::path::PathBuf;
///
/// let path_to_save: PathBuf = PathBuf::from("/example/path/to/save/file/spalst_save");
/// let expected_output: PathBuf = PathBuf::from("/example/path/to/save/file/spalst_save.ron");
/// let output: PathBuf = convert_path(&path_to_save).unwrap();
/// assert_eq!(expected_output, output);
/// ```
pub fn convert_path(path: &Path) -> Result<PathBuf> {
    // Create a variable new_path that will add the extension .ron to the existing path if it ends
    // with spalst_save (so instead of ending with spalst_save, it'll end with spalst_save.ron).
    let new_path: PathBuf;
    if path.ends_with("spalst_save") {
        // Add the extension to the new path if it doesn't exist
        new_path = path.with_extension("ron");
        // If the new path exists and is not a file, return an error.
        if new_path.try_exists().with_note(|| {
            format!(
                "Your user may not have permissions to access the path {}.",
                new_path.display()
            )
        })? && !new_path.is_file()
        {
            bail!(
                "Path {} was passed as a save file location but isn't a file (it was passed without the extension so one was added).",
                new_path.display()
            );
        };
    } else {
        new_path = path.to_path_buf();
    };
    // If the path exists and is not a file, return an error.
    if new_path.try_exists().with_note(|| {
        format!(
            "Your user may not have permissions to access the path {}",
            new_path.display()
        )
    })? && !new_path.is_file()
    {
        bail!(
            "Path {} was passed as a save file location but isn't a file.",
            new_path.display()
        );
    };
    // Ok.
    Ok(new_path)
}
