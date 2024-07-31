//! DICOM files processor
//!

pub mod command;
pub mod error;
pub mod parser;

use std::path::{Path, PathBuf};

use self::error::Result;
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
        let file_obj = dicom_object::open_file(self)?;
        let patient = file_obj.try_into()?;
        let data = DicomData::new(self.to_path_buf(), patient);
        Ok(data)
    }
}

#[cfg(test)]
mod tests {}
