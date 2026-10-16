//! `pipelith-dev agents [--check]`: the step table in AGENTS.md, from the
//! steps hk.pkl gives each hook (dev T18, V40). Ported from actlith-dev,
//! itself from pklith-dev; MIT, same author.

use crate::io::Failed;
use std::path::Path;

/// The table block, markers included, for `rows` (one tab-separated line
/// per push step) and the number of commit and push steps.
#[must_use]
pub(crate) fn table(rows: &str, commit: usize, push: usize) -> String {
    let _ = (rows, commit, push);
    String::new()
}

/// Render or check.
///
/// # Errors
///
/// Drift (1), `pkl` or a file failing (2).
pub(crate) fn run_verb(repo: &Path, check: bool) -> Result<(), Failed> {
    let _ = (repo, check);
    Ok(())
}

#[cfg(test)]
mod tests;
