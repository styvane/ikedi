//! This module implements the parsing logic for DICOM files.
use std::path::Path;

use anyhow::Result;
use dicom::core::Tag;
use dicom::object::{FileDicomObject, InMemDicomObject};

use serde::Serialize;
use walkdir::{DirEntry, WalkDir};

/// Recursively read the directory.
pub fn read_directory(source: impl AsRef<Path>) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(source)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
}

/// DICOM Data type.
#[derive(Debug, Serialize)]
pub struct DicomData<P> {
    /// Absolute path to the DICOM file in the catolog.
    path: P,
    #[serde(flatten)]
    /// Patient data
    pub patient: Patient,
}

/// Patient represents the patient data.
#[derive(Debug, Serialize)]
pub struct Patient {
    /// The patient name
    pub name: String,
    /// The patient ID
    pub id: String,
}

impl TryFrom<FileDicomObject<InMemDicomObject>> for Patient {
    type Error = anyhow::Error;
    fn try_from(file_obj: FileDicomObject<InMemDicomObject>) -> Result<Self> {
        let name = file_obj.element(Tag(0x0010, 0x0010))?.to_str()?.to_string();
        let id = file_obj.element(Tag(0x0010, 0x0020))?.to_str()?.to_string();
        Ok(Self { name, id })
    }
}
impl<P> DicomData<P>
where
    P: AsRef<Path>,
{
    /// Creates new [`DicomData`]
    pub const fn new(path: P, patient: Patient) -> Self {
        Self { path, patient }
    }
}

#[cfg(test)]
mod tests {
    use super::read_directory;
    use crate::DicomFileProcessor;

    #[test]
    fn test_parser() -> anyhow::Result<()> {
        for entry in read_directory("testdata/subject2/") {
            let data = entry.path().open()?;
            insta::assert_debug_snapshot!(data);
        }
        Ok(())
    }
}
