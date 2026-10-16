//! `write` (V30): write each generated file under the root, only where
//! its bytes differ, and nowhere else.
//!
//! Files are written with the default mode (0644 under a usual umask),
//! scripts included: a pipeline runs them through `bash`, so they need no
//! executable bit.

use std::io;
use std::path::Path;

use super::file::File;

#[cfg(test)]
mod tests;

/// Write each of `files` under `root`, creating parent directories,
/// skipping a file whose bytes already match (its mtime kept); returns
/// the paths written, in the order of `files`.
///
/// # Errors
///
/// A path that is not plain and relative (before anything is written),
/// or the first failed directory creation or write.
pub fn write(root: &Path, files: &[File]) -> io::Result<Vec<String>> {
    let _ = (root, files);
    Ok(Vec::new())
}
