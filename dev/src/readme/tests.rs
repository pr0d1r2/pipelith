//! `readme` against fixture repositories: every way it can pass, drift or
//! fail. Needs `pkl` and `git` on PATH, as the gate does.

use super::run_verb;
use std::path::{Path, PathBuf};
use std::process::Command;

const SCHEMA: &str = include_str!("../../../pkl/Config.pkl");
const HK: &str = "amends \"pkl/Config.pkl\"\nlocal fast = new Mapping<String, Step> { [\"a\"] { check = \"true\" } }\nhooks {\n  [\"pre-commit\"] { steps = fast }\n  [\"pre-push\"] { steps = (fast) { [\"b\"] { check = \"true\" } } }\n}\n";
const CARGO: &str = "[package]\nname = \"x\"\nedition = \"2024\"\nrust-version = \"1.95\"\nlicense = \"MIT\"\nrepository = \"https://github.com/o/r\"\n\n[dependencies]\n\n[workspace]\n";
const LOCK: &str = "{\n  \"nodes\": {\n    \"nixpkgs\": {\n      \"locked\": {\n        \"lastModified\": 1790340521,\n        \"rev\": \"f5c082a40f7571c266e74e80ae2e68aadd8a9fc7\"\n      },\n      \"original\": {\n        \"ref\": \"nixos-26.05\"\n      }\n    }\n  }\n}\n";
const CI: &str = "jobs:\n  gate:\n    strategy:\n      matrix:\n        os: [macos-latest]\n";
const DISCLAIMER: &str =
    "**1 invariants**, **1 bugs**, a floor of 100% here.\n";
const README: &str =
    "# x\n\n<!-- BEGIN badges -->\n<!-- END badges -->\n\nbody\n";

/// The fixture's files, whose sources agree with each other.
const FILES: [(&str, &str); 9] = [
    ("pkl/Config.pkl", SCHEMA),
    ("hk.pkl", HK),
    ("Cargo.toml", CARGO),
    (".coverage", "lines 100.00\n"),
    ("flake.lock", LOCK),
    (".github/workflows/ci.yml", CI),
    ("docs/LLM-DISCLAIMER.md", DISCLAIMER),
    ("README.md", README),
    ("SPEC.md", "V1: a\nB1|d|c|f\n"),
];

/// A tracked fixture repository holding `FILES`.
fn fixture(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("pipelith-dev-readme-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for (path, text) in FILES {
        write(&dir, path, text);
    }
    for args in [&["init", "-q"][..], &["add", "-A"]] {
        git(&dir, args);
    }
    dir
}

/// git in the scratch repository: the hook's repository variables
/// dropped and hooks disabled, so a hooked run cannot recurse (nix V47).
fn git(dir: &Path, args: &[&str]) {
    let mut command = Command::new("git");
    for var in crate::io::GIT_REPO_VARS {
        command.env_remove(var);
    }
    let _ = command
        .args(["-c", "core.hooksPath=/dev/null"])
        .args(args)
        .current_dir(dir)
        .output();
}

fn write(dir: &Path, path: &str, text: &str) {
    let path = dir.join(path);
    let _ = std::fs::create_dir_all(path.parent().unwrap_or(dir));
    let _ = std::fs::write(path, text);
}

fn text(dir: &Path, path: &str) -> String {
    std::fs::read_to_string(dir.join(path)).unwrap_or_default()
}

fn failure(got: Result<(), (u8, String)>) -> (u8, String) {
    got.err().unwrap_or_default()
}

/// Writing renders the block in place; a check then passes.
#[test]
fn readme_renders_then_checks() {
    let dir = fixture("render");
    assert_eq!(run_verb(&dir, false), Ok(()));
    let readme = text(&dir, "README.md");
    assert!(
        readme.contains("[![gate steps 1 commit / 2 push]"),
        "{readme}"
    );
    assert!(readme.contains("[![invariants 1]"), "{readme}");
    assert!(readme.contains("[![direct dependencies 0]"), "{readme}");
    assert!(
        readme.ends_with("<!-- END badges -->\n\nbody\n"),
        "{readme}"
    );
    assert_eq!(run_verb(&dir, true), Ok(()));
}

/// A hand-edited badge is drift: `--check` fails, naming the fix.
#[test]
fn an_edited_badge_drifts() {
    let dir = fixture("drift");
    let _ = run_verb(&dir, false);
    let edited = text(&dir, "README.md").replace("MSRV 1.95", "MSRV 1.0");
    write(&dir, "README.md", &edited);
    let (code, why) = failure(run_verb(&dir, true));
    assert_eq!(code, 1);
    assert!(
        why.starts_with("README.md drifted from its sources"),
        "{why}"
    );
}

/// The disclaimer's numbers must match the sources, or nothing is written.
#[test]
fn stale_disclaimer_prose_fails() {
    let dir = fixture("prose");
    let stale = DISCLAIMER.replace("**1 invariants", "**9 invariants");
    write(&dir, "docs/LLM-DISCLAIMER.md", &stale);
    let (code, why) = failure(run_verb(&dir, false));
    assert_eq!(code, 1);
    assert!(why.contains("must say **1 invariants**"), "{why}");
    assert_eq!(text(&dir, "README.md"), README);
}

/// A source replaced by text that holds no fact, and the error it names.
const BROKEN: [(&str, &str, &str); 8] = [
    (
        "README.md",
        "# no block\n",
        "README.md has no <!-- BEGIN badges -->",
    ),
    (
        ".coverage",
        "nothing\n",
        ".coverage holds no readable value",
    ),
    (
        "flake.lock",
        "{}\n",
        "flake.lock nixpkgs holds no readable value",
    ),
    (
        ".github/workflows/ci.yml",
        "x: 1\n",
        "ci.yml has no `os: [...]`",
    ),
    ("Cargo.toml", "[package]\n", "has no [dependencies] table"),
    ("Cargo.toml", "[dependencies]\n", "read empty or zero"),
    ("hk.pkl", "not pkl\n", "pkl eval -x"),
    ("SPEC.md", "no rows\n", "SPEC.md rows read empty or zero"),
];

/// Unreadable sources and missing facts are exit 2, named.
#[test]
fn broken_sources_exit_2() {
    for (i, (path, body, want)) in BROKEN.iter().enumerate() {
        let dir = fixture(&format!("broken-{i}"));
        write(&dir, path, body);
        let (code, why) = failure(run_verb(&dir, true));
        assert!(code == 2 && why.contains(want), "{path}: {why}");
    }
}

/// A missing source is exit 2, naming the file.
#[test]
fn a_missing_source_exits_2() {
    let dir = fixture("missing");
    let _ = std::fs::remove_file(dir.join(".coverage"));
    let (code, why) = failure(run_verb(&dir, false));
    assert!(code == 2 && why.contains("cannot read .coverage"), "{why}");
}

/// A README that cannot be written is exit 2.
#[test]
fn an_unwritable_readme_exits_2() {
    use std::os::unix::fs::PermissionsExt as _;
    let dir = fixture("readonly");
    let ro = std::fs::Permissions::from_mode(0o444);
    let _ = std::fs::set_permissions(dir.join("README.md"), ro);
    let (code, why) = failure(run_verb(&dir, false));
    assert!(code == 2 && why.contains("cannot write README.md"), "{why}");
}
