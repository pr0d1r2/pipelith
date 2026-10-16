//! `agents`: the table from text, then the verb against a fixture hk.pkl
//! evaluated by the real `pkl`.

use super::{run_verb, table};
use std::path::{Path, PathBuf};

const SCHEMA: &str = include_str!("../../../pkl/Config.pkl");
const HK: &str = "amends \"pkl/Config.pkl\"\nlocal fast = new Mapping<String, Step> { [\"a\"] { glob = List(\"*.rs\"); check = \"true\"; fix = \"true\" } }\nhooks {\n  [\"pre-commit\"] { steps = fast }\n  [\"pre-push\"] { steps = (fast) { [\"b\"] { check = \"true\" } } }\n}\n";
const AGENTS: &str =
    "# Agents\n\nhand text\n\n<!-- BEGIN steps -->\n<!-- END steps -->\n";

#[test]
fn rows_become_the_table() {
    let rows = "a\tcommit\t**/*.rs\u{1F}Cargo.toml\tyes\nb\tpush\t\t-\n";
    let got = table(rows, 1, 2);
    assert!(
        got.contains("1 steps run on every commit and 2 on push"),
        "{got}"
    );
    assert!(got.contains("| `a` | commit | `**/*.rs` `Cargo.toml` | yes |\n| `b` | push | whole tree | - |\n<!-- END steps -->\n"), "{got}");
}

fn fixture(name: &str, agents: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("pipelith-dev-agents-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::create_dir_all(dir.join("pkl"));
    write(&dir, "pkl/Config.pkl", SCHEMA);
    write(&dir, "hk.pkl", HK);
    write(&dir, "AGENTS.md", agents);
    dir
}

fn write(dir: &Path, path: &str, text: &str) {
    let _ = std::fs::write(dir.join(path), text);
}

fn text(dir: &Path) -> String {
    std::fs::read_to_string(dir.join("AGENTS.md")).unwrap_or_default()
}

/// The table renders from hk.pkl; a check passes; a hand edit drifts.
#[test]
fn the_table_renders_then_checks() {
    let dir = fixture("render", AGENTS);
    assert_eq!(run_verb(&dir, false), Ok(()));
    let got = text(&dir);
    let want =
        "| `a` | commit | `*.rs` | yes |\n| `b` | push | whole tree | - |\n";
    assert!(got.contains(want) && got.contains("hand text"), "{got}");
    assert_eq!(run_verb(&dir, true), Ok(()));
    write(&dir, "AGENTS.md", &got.replace("| yes |", "| - |"));
    assert_eq!(run_verb(&dir, true).err().map(|e| e.0), Some(1));
}

/// An AGENTS.md without the block, and an hk.pkl pkl rejects, are exit 2.
#[test]
fn a_missing_block_or_bad_hk_exits_2() {
    let dir = fixture("block", "# Agents\n");
    let (code, why) = run_verb(&dir, true).err().unwrap_or_default();
    assert!(
        code == 2 && why.contains("no <!-- BEGIN steps -->"),
        "{why}"
    );
    write(&dir, "hk.pkl", "not pkl\n");
    assert_eq!(run_verb(&dir, true).err().map(|e| e.0), Some(2));
}
