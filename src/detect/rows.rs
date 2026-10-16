//! `detect`: proposed `## workflows` rows from tracked paths and
//! `flake.lock` text, in fixed order (V35, `.:V1`).

use super::lock::has_node;
use super::text::FIRST_ROW;
use crate::kind::{GATE, RawRow, UPDATE_PINS};

/// The id of the proposed pin-bump row (detect I).
const PINS: &str = "update-pins";

/// Proposed rows: `flake.nix` tracked gives `ci|gate`; a `nixpkgs-lock`
/// node in `flake.lock` gives `update-pins|update-pins`. Always that
/// order; each row's `line` is the one it takes in the seeded file.
#[must_use]
pub fn detect<S: AsRef<str>>(
    tracked: &[S],
    flake_lock: Option<&str>,
) -> Vec<RawRow> {
    let gate = tracked.iter().any(|path| path.as_ref() == "flake.nix");
    let pins = flake_lock.is_some_and(|lock| has_node(lock, "nixpkgs-lock"));
    let found = [(gate, "ci", GATE.name), (pins, PINS, UPDATE_PINS.name)];
    found
        .into_iter()
        .filter(|&(hit, _, _)| hit)
        .zip(FIRST_ROW..)
        .map(|((_, id, kind), line)| bare(line, id, kind))
        .collect()
}

/// A row with every optional cell empty: the kind's defaults.
fn bare(line: usize, id: &str, kind: &str) -> RawRow {
    RawRow {
        line,
        id: id.to_owned(),
        kind: kind.to_owned(),
        ..RawRow::default()
    }
}

#[cfg(test)]
mod tests;
