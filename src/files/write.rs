//! `write` (V30): write each generated file under the root, only where
//! its bytes differ, and nowhere else.
//!
//! Files are written with the default mode (0644 under a usual umask),
//! scripts included: a pipeline runs them through `bash`, so they need no
//! executable bit.

use std::fs;
use std::io;
use std::path::Path;

use super::file::File;
use super::target::target;

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
    let targets = files
        .iter()
        .map(|file| target(root, &file.path))
        .collect::<io::Result<Vec<_>>>()?;
    let mut written = Vec::new();
    for (file, path) in files.iter().zip(targets) {
        if write_if_changed(&path, file.bytes.as_bytes())? {
            written.push(file.path.clone());
        }
    }
    Ok(written)
}

/// Write `bytes` at `path` unless it already holds them; true if written.
fn write_if_changed(path: &Path, bytes: &[u8]) -> io::Result<bool> {
    if fs::read(path).is_ok_and(|old| old == bytes) {
        return Ok(false);
    }
    // `target` joins at least one segment, so a parent always exists.
    path.parent().map_or(Ok(()), fs::create_dir_all)?;
    fs::write(path, bytes)?;
    Ok(true)
}
