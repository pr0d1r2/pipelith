//! `has_node` against real-shaped locks and edge text (T16, V35).

use super::has_node;

const WITH: &str =
    include_str!("../../../tests/fixtures/detect/with-lock.lock");
const WITHOUT: &str =
    include_str!("../../../tests/fixtures/detect/no-lock.lock");

#[test]
fn a_lock_with_the_node_has_it() {
    assert!(has_node(WITH, "nixpkgs-lock"));
    assert!(has_node(WITH, "hk"));
}

#[test]
fn a_lock_without_the_node_lacks_it() {
    assert!(!has_node(WITHOUT, "nixpkgs-lock"));
    assert!(has_node(WITHOUT, "nixpkgs"));
}

#[test]
fn input_maps_and_follows_lists_are_not_nodes() {
    assert!(!has_node(
        r#"{"inputs": {"nixpkgs-lock": "x"}}"#,
        "nixpkgs-lock"
    ));
    assert!(!has_node(r#"{"nixpkgs-lock" : ["x"]}"#, "nixpkgs-lock"));
    assert!(!has_node(r#"["nixpkgs-lock"]"#, "nixpkgs-lock"));
}

#[test]
fn only_the_exact_name_matches() {
    assert!(!has_node(r#"{"nixpkgs-lock_2": {}}"#, "nixpkgs-lock"));
    assert!(!has_node(r#"{"my-nixpkgs-lock": {}}"#, "nixpkgs-lock"));
    assert!(has_node("{\"nixpkgs-lock\"\n  :\n  {}}", "nixpkgs-lock"));
}

#[test]
fn empty_text_has_no_node() {
    assert!(!has_node("", "nixpkgs-lock"));
}
