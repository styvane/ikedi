//! This module define the command line interface.
//!
use clap::Parser;

/// A tool that catalogs Digital Imaging and Communications in Medicine files.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path represents the directory path to the DICOM files.
    pub path: String,

    /// Backpressure control
    #[arg(short, long, default_value_t = 1000)]
    pub ratelimit: usize,
}

impl Cli {
    /// Create new Cli with parsed arguments.
    pub fn new() -> Self {
        Self::parse()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
