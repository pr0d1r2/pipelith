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
pub(crate) fn check(at: Pos<'_>, text: &str) -> Result<u32, RegistryError> {
    let mut words = text.split_whitespace();
    if words.next() != Some("format") {
        let fix = format!("start the file with `format {SUPPORTED_FORMAT}`");
        return Err(at.error("no `format` line", fix));
    }
    version(at, &words.collect::<Vec<_>>().join(" "))
}

/// The version after `format` is the supported one.
fn version(at: Pos<'_>, version: &str) -> Result<u32, RegistryError> {
    match version.parse::<u32>() {
        Ok(SUPPORTED_FORMAT) => Ok(SUPPORTED_FORMAT),
        Ok(newer) if newer > SUPPORTED_FORMAT => Err(too_new(at, newer)),
        _ => {
            let what = format!("unknown format `{version}`");
            let fix = format!("write `format {SUPPORTED_FORMAT}`");
            Err(at.error(what, fix))
        }
    }
}

/// A newer format names both versions; no best-effort parse (V15).
fn too_new(at: Pos<'_>, newer: u32) -> RegistryError {
    let what = format!(
        "`{}` is format {newer}; this build reads format {SUPPORTED_FORMAT}",
        at.file
    );
    let fix = format!("upgrade to a release that reads format {newer}");
    at.error(what, fix)
}

#[cfg(test)]
mod tests;
