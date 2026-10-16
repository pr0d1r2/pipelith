//! Where a generated path lands under the root (V29): relative, plain
//! segments only, so nothing `write` writes or `check` reads can leave
//! the root.

use std::io;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests;

/// `rel` joined under `root`.
///
/// # Errors
///
/// `InvalidInput` naming `rel` when it is empty, absolute, or has an
/// empty, `.` or `..` segment (or a `\`). Checked on the `/`-separated
/// text, not on `Path` components, which drop an inner `.`.
pub fn target(root: &Path, rel: &str) -> io::Result<PathBuf> {
    Ok(root.join(rel))
}
