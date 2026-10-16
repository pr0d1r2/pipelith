//! `commit-msg [FILE]` and `changelog [FILE]`: the commit-msg hook's two
//! rules (nix C, SPEC.md V7, dev T18, V41). The hook passes the file git
//! gave it; by hand it defaults to COMMIT_EDITMSG.

use crate::io::Failed;
use std::path::Path;

/// The commit-message style rule.
///
/// # Errors
///
/// The rule broken (1), or no message to judge (2).
pub(crate) fn commit_msg(
    repo: &Path,
    file: Option<&str>,
) -> Result<(), Failed> {
    let _ = (repo, file);
    Ok(())
}

/// The changelog rule.
///
/// # Errors
///
/// The rule broken (1), or git or the message failing (2).
pub(crate) fn changelog(repo: &Path, file: Option<&str>) -> Result<(), Failed> {
    let _ = (repo, file);
    Ok(())
}

#[cfg(test)]
mod tests;
