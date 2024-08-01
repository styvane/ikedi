//! # Ekedi
//!
//! DICOM files processor

pub mod command;
pub mod parser;
pub mod processor;

use std::path::{Path, PathBuf};

use anyhow::Result;

use self::parser::DicomData;

/// DicomProcessor defines the various operations that
/// can be performed on DICOM files
pub trait DicomFileProcessor {
    /// Catalog root path.
    type CatalogPath;
    /// Creates a DIDCOM data from the DICOM file.
    fn open(&self) -> Result<DicomData<Self::CatalogPath>>;
}

impl DicomFileProcessor for Path {
    type CatalogPath = PathBuf;

    fn open(&self) -> Result<DicomData<PathBuf>> {
        let file_obj = dicom::object::open_file(self)?;
        let patient = file_obj.try_into()?;
        let data = DicomData::new(self.to_path_buf(), patient);
        Ok(data)
    }
}

#[cfg(test)]
mod tests {}
