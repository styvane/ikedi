//! This module defines the error type.
//!
use std::error::Error;

/// The result type is a type alias for [`std::result::Result`]
/// with `Box<dyn std::error::Error>` as error.
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
