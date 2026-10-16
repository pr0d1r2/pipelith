//! `notices`: the pure rules over text, then the verb against a fixture
//! crate named `pipelith` with one path dependency, locked offline.

use super::{crates, run_verb, stale_prose, table};
use std::path::{Path, PathBuf};
use std::process::Command;

const TREE: &str = "pipelith v0.0.0 (/x/pipelith)|MIT\nglobset v0.4.20|Unlicense OR MIT\nmemchr v2.8.3|Unlicense OR MIT\nglobset v0.4.20|Unlicense OR MIT (*)\nbroken line\nnoversion|MIT\n";

#[test]
fn crates_come_from_cargo_tree_once_each() {
    let got = crates(TREE, "pipelith");
    let names: Vec<&str> = got.iter().map(|(n, _, _)| n.as_str()).collect();
    assert_eq!(names, ["globset", "memchr"]);
    assert!(table(&got).contains("| globset | 0.4.20 | Unlicense OR MIT |\n"));
}

/// No dependencies is a table with no rows, not an error.
#[test]
fn no_crates_is_an_empty_table() {
    let got = table(&crates("pipelith v0.0.0 (/x)|MIT\n", "pipelith"));
    assert!(
        got.ends_with("| --- | --- | --- |\n<!-- END crates -->\n"),
        "{got}"
    );
    assert!(
        stale_prose(
            "**0 direct dependencies**, which pulls in 0 crates",
            0,
            &[]
        )
        .is_empty()
    );
}

#[test]
fn the_prose_is_held_to_the_table() {
    let set = crates(TREE, "pipelith");
    let doc =
        "ships **1 direct dependency**, which pulls in 2\ncrates in total.";
    assert!(stale_prose(doc, 1, &set).is_empty());
    assert_eq!(stale_prose(doc, 2, &set).len(), 1);
    let gpl = vec![("x".into(), "1".into(), "GPL-3.0".into())];
    let doc = "**2 direct dependencies**, which pulls in 1 crates";
    assert_eq!(
        stale_prose(doc, 2, &gpl),
        [
            "x is GPL-3.0, not available under MIT as docs/THIRD-PARTY-NOTICES.md says"
        ]
    );
}

const CARGO: &str = "[package]\nname = \"pipelith\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\nleaf = { path = \"leaf\" }\n\n[workspace]\n";
const LEAF: &str = "[package]\nname = \"leaf\"\nversion = \"0.1.0\"\nedition = \"2024\"\nlicense = \"MIT\"\n";
const DOC: &str = "# Notices\n\nships **1 direct dependency**, which pulls in 1 crates.\n\n<!-- BEGIN crates -->\n<!-- END crates -->\n\nMIT.\n";

const FILES: [(&str, &str); 5] = [
    ("Cargo.toml", CARGO),
    ("src/lib.rs", ""),
    ("leaf/Cargo.toml", LEAF),
    ("leaf/src/lib.rs", ""),
    ("docs/THIRD-PARTY-NOTICES.md", DOC),
];

/// A locked crate with one path dependency, and its notices doc.
fn fixture(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "pipelith-dev-notices-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    for (path, text) in FILES {
        write(&dir, path, text);
    }
    lock(&dir);
    dir
}

/// `cargo generate-lockfile`, offline, with the hook's repository
/// variables dropped (nix V47).
fn lock(dir: &Path) {
    let mut cargo = Command::new("cargo");
    for var in crate::io::GIT_REPO_VARS {
        cargo.env_remove(var);
    }
    let _ = cargo
        .args(["generate-lockfile", "--offline", "-q"])
        .current_dir(dir)
        .output();
}

fn write(dir: &Path, path: &str, text: &str) {
    let path = dir.join(path);
    let _ = std::fs::create_dir_all(path.parent().unwrap_or(dir));
    let _ = std::fs::write(path, text);
}

fn doc(dir: &Path) -> String {
    std::fs::read_to_string(dir.join("docs/THIRD-PARTY-NOTICES.md"))
        .unwrap_or_default()
}

fn failure(got: Result<(), (u8, String)>) -> (u8, String) {
    got.err().unwrap_or_default()
}

/// The table renders from the lock; a check passes; an edited row drifts.
#[test]
fn the_table_renders_then_checks() {
    let dir = fixture("render");
    assert_eq!(run_verb(&dir, false), Ok(()));
    let got = doc(&dir);
    assert!(got.contains("| leaf | 0.1.0 | MIT |\n"), "{got}");
    assert_eq!(run_verb(&dir, true), Ok(()));
    let edited = got.replace("0.1.0", "9");
    write(&dir, "docs/THIRD-PARTY-NOTICES.md", &edited);
    let (code, why) = failure(run_verb(&dir, true));
    assert_eq!(code, 1);
    assert!(why.contains("drifted from its sources"), "{why}");
}

/// A count the prose gets wrong is exit 1, before anything is written.
#[test]
fn a_wrong_count_in_the_prose_fails() {
    let dir = fixture("count");
    let wrong = DOC.replace("1 crates", "3 crates");
    write(&dir, "docs/THIRD-PARTY-NOTICES.md", &wrong);
    let (code, why) = failure(run_verb(&dir, false));
    assert_eq!(code, 1);
    assert!(why.contains("must say **1 direct dependency**"), "{why}");
}

/// A doc without the block, a nameless manifest, and cargo failing are 2.
#[test]
fn a_missing_block_name_or_lock_exits_2() {
    let dir = fixture("block");
    let bare = "**1 direct dependency**, which pulls in 1 crates\n";
    write(&dir, "docs/THIRD-PARTY-NOTICES.md", bare);
    let (code, why) = failure(run_verb(&dir, true));
    assert!(
        code == 2 && why.contains("no <!-- BEGIN crates -->"),
        "{why}"
    );
    let _ = std::fs::remove_file(dir.join("Cargo.lock"));
    let (code, why) = failure(run_verb(&dir, true));
    assert!(code == 2 && why.contains("cargo tree"), "{why}");
    write(&dir, "Cargo.toml", "[workspace]\n");
    let (code, why) = failure(run_verb(&dir, true));
    assert!(code == 2 && why.contains("no package name"), "{why}");
}
