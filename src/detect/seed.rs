//! The first config file: written once, never over an existing one
//! (V36, `actlith/V32`).

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use crate::kind::RawRow;

/// Why [`seed_init`] wrote nothing.
#[derive(Debug)]
pub enum SeedError {
    /// The file is already there; it is left as it was (V36, exit 1).
    Exists(PathBuf),
    /// The file could not be written (exit 2, unwritable).
    Write(PathBuf, io::Error),
}

impl SeedError {
    /// The exit code a forge CLI returns: 1 finding, 2 unwritable.
    #[must_use]
    pub fn code(&self) -> u8 {
        let _ = self;
        0
    }
}

impl fmt::Display for SeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = self;
        f.write_str("")
    }
}

impl std::error::Error for SeedError {}

/// Write `<root>/<config>` holding `rows` (from
/// [`detect`](super::detect())) as [`text`](super::text()); returns its
/// path. The forge crate names its config file (`.actlith`,
/// `.lablith`).
///
/// # Errors
///
/// [`SeedError::Exists`] if the file is there (it is not touched), else
/// [`SeedError::Write`] with the I/O error.
pub fn seed_init(
    root: &Path,
    config: &str,
    rows: &[RawRow],
) -> Result<PathBuf, SeedError> {
    let _ = rows;
    Err(SeedError::Exists(root.join(config)))
}

#[cfg(test)]
mod tests;
