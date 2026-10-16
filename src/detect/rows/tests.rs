//! `detect` per tracked-file shape (T16, V35).

use super::detect;
use crate::kind::RawRow;

const WITH: &str =
    include_str!("../../../tests/fixtures/detect/with-lock.lock");
const WITHOUT: &str =
    include_str!("../../../tests/fixtures/detect/no-lock.lock");

fn shape(rows: &[RawRow]) -> Vec<(usize, &str, &str)> {
    rows.iter()
        .map(|r| (r.line, r.id.as_str(), r.kind.as_str()))
        .collect()
}

#[test]
fn nothing_tracked_proposes_nothing() {
    let none: [&str; 0] = [];
    assert!(detect(&none, None).is_empty());
    assert!(detect(&["README.md", "src/flake.nix"], None).is_empty());
}

#[test]
fn flake_nix_proposes_the_gate() {
    let rows = detect(&["README.md", "flake.nix"], Some(WITHOUT));
    assert_eq!(shape(&rows), [(5, "ci", "gate")]);
}

#[test]
fn a_nixpkgs_lock_node_proposes_update_pins() {
    let rows = detect(&["flake.lock"], Some(WITH));
    assert_eq!(shape(&rows), [(5, "update-pins", "update-pins")]);
}

#[test]
fn both_come_gate_first_with_empty_cells() {
    let tracked = vec!["flake.lock".to_owned(), "flake.nix".to_owned()];
    let rows = detect(&tracked, Some(WITH));
    let want = [(5, "ci", "gate"), (6, "update-pins", "update-pins")];
    assert_eq!(shape(&rows), want);
    let empty = |r: &RawRow| {
        r.systems.is_empty() && r.cache.is_none() && r.args.is_empty()
    };
    assert!(rows.iter().all(empty));
}

#[test]
fn the_kinds_are_in_the_catalogue() {
    let rows = detect(&["flake.nix"], Some(WITH));
    let known = |r: &RawRow| crate::kind::lookup(&r.kind).is_some();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(known));
}

#[test]
fn same_input_same_rows() {
    let tracked = ["flake.nix", "flake.lock"];
    assert_eq!(detect(&tracked, Some(WITH)), detect(&tracked, Some(WITH)));
}
