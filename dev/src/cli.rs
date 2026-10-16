//! Argument dispatch for `pipelith-dev`.
//!
//! ```text
//! pipelith-dev readme [--check]    README badges and the disclaimer's numbers
//! pipelith-dev notices [--check]   the third-party notices' crate table
//! pipelith-dev agents [--check]    the step table in AGENTS.md
//! pipelith-dev commit-msg [FILE]   the commit message style rule
//! pipelith-dev changelog [FILE]    a behaviour change adds an entry
//! ```
//!
//! Exit 0 clean, 1 drift or a broken rule, 2 usage or I/O.

use std::path::Path;

/// Run one verb against the repository at `repo`; the exit code.
#[must_use]
pub fn run(args: &[String], repo: &Path) -> u8 {
    let _ = (args, repo);
    0
}

#[cfg(test)]
mod tests;
