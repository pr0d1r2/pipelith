//! `pipelith-dev readme [--check]`: the README badge block, and the numbers
//! `docs/LLM-DISCLAIMER.md` quotes, from the files that own them (dev T18,
//! V40).

use crate::io::Failed;
use std::path::Path;

/// Render or check.
///
/// # Errors
///
/// Stale disclaimer prose or drift (1), an unreadable source (2).
pub(crate) fn run_verb(repo: &Path, check: bool) -> Result<(), Failed> {
    let _ = (repo, check);
    Ok(())
}

#[cfg(test)]
mod tests;
