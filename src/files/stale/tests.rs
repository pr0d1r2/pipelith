//! `check` over a temp root: names each stale or missing file, sorted by
//! path, and writes nothing (V28).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{Stale, check};
use crate::files::{File, write};

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// A fresh, empty directory for one test, removed when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let id = std::process::id();
        let leaf = format!("pipelith-files-stale-{id}-{n}-{name}");
        let dir = std::env::temp_dir().join(leaf);
        let _ = fs::remove_dir_all(&dir);
        assert!(fs::create_dir_all(&dir).is_ok());
        Self(dir)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn file(path: &str, bytes: &str) -> File {
    File {
        path: path.to_owned(),
        bytes: bytes.to_owned(),
    }
}

/// Two files, listed out of path order.
fn two() -> Vec<File> {
    vec![
        file("scripts/tool/open-pr.sh", "echo pr\n"),
        file("ci/pipelines/build.yml", "name: ci\n"),
    ]
}

/// Every entry under `dir`, sorted; to prove a check left it untouched.
fn tree(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut out = Vec::new();
    let entries = fs::read_dir(dir).map(Iterator::collect::<Vec<_>>);
    for entry in entries.unwrap_or_default().into_iter().flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.push((path.clone(), Vec::new()));
            out.extend(tree(&path));
        } else {
            out.push((path.clone(), fs::read(&path).unwrap_or_default()));
        }
    }
    out.sort();
    out
}

#[test]
fn names_every_missing_file_sorted_and_creates_nothing() {
    let root = Scratch::new("missing");
    let want = vec![
        Stale::Missing("ci/pipelines/build.yml".to_owned()),
        Stale::Missing("scripts/tool/open-pr.sh".to_owned()),
    ];
    assert_eq!(check(&root.0, &two()).ok(), Some(want));
    assert_eq!(tree(&root.0), Vec::new());
}

#[test]
fn names_a_changed_file_and_leaves_it_as_it_was() {
    let root = Scratch::new("changed");
    assert!(write(&root.0, &two()).is_ok());
    let ci = root.0.join("ci/pipelines/build.yml");
    assert!(fs::write(&ci, "hand edit\n").is_ok());
    let before = tree(&root.0);
    let want = vec![Stale::Changed("ci/pipelines/build.yml".to_owned())];
    assert_eq!(check(&root.0, &two()).ok(), Some(want));
    assert_eq!(tree(&root.0), before);
}

#[test]
fn sorts_changed_and_missing_together_by_path() {
    let root = Scratch::new("mixed");
    let files = [file("b.yml", "b\n"), file("a.yml", "a\n")];
    assert!(fs::write(root.0.join("b.yml"), "edited\n").is_ok());
    let want = vec![
        Stale::Missing("a.yml".to_owned()),
        Stale::Changed("b.yml".to_owned()),
    ];
    assert_eq!(check(&root.0, &files).ok(), Some(want));
}

#[test]
fn finds_nothing_after_write() {
    let root = Scratch::new("current");
    assert!(write(&root.0, &two()).is_ok());
    let before = tree(&root.0);
    assert_eq!(check(&root.0, &two()).ok(), Some(Vec::new()));
    assert_eq!(tree(&root.0), before);
}

#[test]
fn refuses_an_escaping_path() {
    let root = Scratch::new("escape");
    assert!(check(&root.0, &[file("/etc/passwd", "x\n")]).is_err());
}

#[test]
fn reports_a_path_it_cannot_read() {
    let root = Scratch::new("unreadable");
    let at = root.0.join("ci/pipelines/build.yml");
    assert!(fs::create_dir_all(at).is_ok());
    assert!(check(&root.0, &two()).is_err());
}

#[test]
fn names_each_kind_and_path_when_shown() {
    let missing = Stale::Missing("a.yml".to_owned()).to_string();
    let changed = Stale::Changed("b.yml".to_owned()).to_string();
    assert_eq!(missing, "missing: a.yml");
    assert_eq!(changed, "stale: b.yml");
}
