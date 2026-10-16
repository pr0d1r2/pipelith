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
    if rel.split('/').all(plain) {
        Ok(root.join(rel))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("`{rel}`: not a plain relative path under the root"),
        ))
    }
}

/// One segment that names a file or directory, nothing else.
fn plain(segment: &str) -> bool {
    !matches!(segment, "" | "." | "..") && !segment.contains('\\')
}
