//! `check` (V28): compare each generated file with the one under the
//! root; writes nothing.

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use super::file::File;
use super::target::target;

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
        match self {
            Self::Missing(path) => write!(f, "missing: {path}"),
            Self::Changed(path) => write!(f, "stale: {path}"),
        }
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
    let mut stale = Vec::new();
    for file in files {
        stale.extend(compare(root, file)?);
    }
    stale.sort_by(|a, b| path(a).cmp(path(b)));
    Ok(stale)
}

/// How `file` stands under `root`: `None` when current.
fn compare(root: &Path, file: &File) -> io::Result<Option<Stale>> {
    match fs::read(target(root, &file.path)?) {
        Ok(bytes) if bytes == file.bytes.as_bytes() => Ok(None),
        Ok(_) => Ok(Some(Stale::Changed(file.path.clone()))),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            Ok(Some(Stale::Missing(file.path.clone())))
        }
        Err(err) => Err(err),
    }
}

/// The path a stale entry names.
fn path(stale: &Stale) -> &str {
    match stale {
        Stale::Missing(path) | Stale::Changed(path) => path,
    }
}
