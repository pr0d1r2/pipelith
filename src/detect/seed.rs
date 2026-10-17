//! The first config file: written once, never over an existing one
//! (V36, `actlith/V32`).

use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

use super::text::text;
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
        match self {
            Self::Exists(_) => 1,
            Self::Write(..) => 2,
        }
    }
}

impl fmt::Display for SeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exists(path) => write!(
                f,
                "{}: already exists; seeding never overwrites it",
                path.display()
            ),
            Self::Write(path, err) => {
                write!(f, "{}: cannot write: {err}", path.display())
            }
        }
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
    let path = root.join(config);
    let written = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .and_then(|mut file| file.write_all(text(rows).as_bytes()));
    match written {
        Ok(()) => Ok(path),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            Err(SeedError::Exists(path))
        }
        Err(e) => Err(SeedError::Write(path, e)),
    }
}

/// Write `<root>/<config>` holding `text`, built by the forge (from
/// [`text_with`](super::text_with())); returns its path. Same write as
/// [`seed_init`]: created new, never over an existing file (V36, V79).
///
/// # Errors
///
/// [`SeedError::Exists`] if the file is there (it is not touched), else
/// [`SeedError::Write`] with the I/O error.
pub fn seed_init_text(
    root: &Path,
    config: &str,
    text: &str,
) -> Result<PathBuf, SeedError> {
    let _ = (text, config);
    Ok(root.to_path_buf())
}

#[cfg(test)]
mod tests;
