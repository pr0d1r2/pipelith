//! `detect`: proposed `## workflows` rows from tracked paths and
//! `flake.lock` text, in fixed order (V35, `.:V1`).

use crate::kind::RawRow;

/// Proposed rows: `flake.nix` tracked gives `ci|gate`; a `nixpkgs-lock`
/// node in `flake.lock` gives `update-pins|update-pins`. Always that
/// order; each row's `line` is the one it takes in the seeded file.
#[must_use]
pub fn detect<S: AsRef<str>>(
    tracked: &[S],
    flake_lock: Option<&str>,
) -> Vec<RawRow> {
    let _ = (tracked, flake_lock);
    Vec::new()
}

#[cfg(test)]
mod tests;
