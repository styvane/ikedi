//! This module implements the parsing logic for DICOM files

use std::error::Error;
use std::path::Path;

use dicom_object::{FileDicomObject, InMemDicomObject};
use walkdir::{DirEntry, WalkDir};

use crate::error::Result;

/// Recursively read the directory.
pub fn read_directory(source: impl AsRef<Path>) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(source)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
}

/// DICOM Data type.
pub struct DicomData<P> {
    /// Absolute path to the DICOM data in the catolog.
    path: P,
    /// Patient data
    pub patient: Patient,
}

/// Patient represents the patient data.
pub struct Patient {
    /// The patient name
    pub name: String,
    /// The patient ID
    pub id: String,
}

impl TryFrom<FileDicomObject<InMemDicomObject>> for Patient {
    type Error = Box<dyn Error>;
    fn try_from(file_obj: FileDicomObject<InMemDicomObject>) -> Result<Self> {
        let name = file_obj
            .element_by_name("PATIENT_NAME")?
            .to_str()?
            .to_string();
        let id = file_obj
            .element_by_name("PATIENT_ID")?
            .to_str()?
            .to_string();
        Ok(Self { name, id })
    }
}
impl<P> DicomData<P>
where
    P: AsRef<Path>,
{
    /// Returns the path to the DICOM data in the catalog
    pub fn path(&self) -> impl AsRef<Path> + '_ {
        &self.path
    }

    /// Creates new [`DicomData`]
    pub const fn new(path: P, patient: Patient) -> Self {
        Self { path, patient }
    }
}

#[cfg(test)]
mod tests {}
