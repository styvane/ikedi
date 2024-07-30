//! DICOM files processor
//!

pub mod command;
pub mod error;
pub mod parser;

/// DicomProcessor defines the various operations that
/// can be performed on DICOM files.
pub trait DicomProcessor {}
