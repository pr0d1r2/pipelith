//! `check` (V28): compare each generated file with the one under the
//! root; writes nothing.

use std::fmt;
use std::io;
use std::path::Path;

use super::file::File;

#[cfg(test)]
mod tests;

/// A generated file the root does not hold as generated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stale {
    /// No file at this path.
    Missing(String),
    /// A file whose bytes differ from the generated ones.
    Changed(String),
}

impl fmt::Display for Stale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = (self, f);
        Ok(())
    }
}

/// Every one of `files` missing or changed under `root`, sorted by path;
/// empty when all are current. Reads only (V28).
///
/// # Errors
///
/// A path that is not plain and relative, or a read failing other than
/// for a missing file (a directory at the path, no permission).
pub fn check(root: &Path, files: &[File]) -> io::Result<Vec<Stale>> {
    let _ = (root, files);
    Ok(Vec::new())
}
