use std::io;
use thiserror::Error;

/// Errors that can occur when working with Glyphs files.
#[derive(Error, Debug)]
pub enum Error {
    /// An I/O error occurred.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// An error occurred while parsing a Glyphs file.
    #[error("Parse error: {0}")]
    Parse(#[from] openstep_plist::Error),

    /// An error occurred during serialization or deserialization.
    #[error("Serialization/Deserialization error: {0}")]
    Serde(#[from] serde_path_to_error::Error<openstep_plist::Error>),

    /// Attempted to save a Glyphs 2 format file as a glyphspackage, which is not supported.
    #[error("Glyphs 2 format files cannot be saved as a glyphspackage")]
    Glyphs2NoPackage,
}

pub type Result<T> = std::result::Result<T, Error>;
