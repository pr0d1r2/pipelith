//! The `format N` line every registry file starts with (registry V15).

use super::error::{Pos, RegistryError};

/// The newest format this grammar reads.
pub const SUPPORTED_FORMAT: u32 = 1;

/// The first content line, `text` at `at`, is `format 1`; its version.
///
/// # Errors
///
/// No format line, a malformed one, or a version newer than
/// [`SUPPORTED_FORMAT`], named next to it (registry V15).
pub(crate) fn check(at: Pos<'_>, _text: &str) -> Result<u32, RegistryError> {
    Err(at.error("", ""))
}

#[cfg(test)]
mod tests;
