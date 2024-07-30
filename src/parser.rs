//! This module implement the parsing logic for DICOM files

use std::path::Path;

use walkdir::DirEntry;
use walkdir::WalkDir;

/// Recursively read the directory.
pub fn read_directory(path: impl AsRef<Path>) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
}
