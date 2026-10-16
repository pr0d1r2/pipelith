//! `seed_init` in a temp dir: writes once, never over (T16, V36).

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{SeedError, seed_init};
use crate::detect::{detect, text};

const CONFIG: &str = ".lablith";

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// A fresh, empty directory for one test, removed when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let id = std::process::id();
        let leaf = format!("pipelith-detect-seed-{id}-{n}-{name}");
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

#[test]
fn writes_the_detected_rows_when_absent() {
    let dir = Scratch::new("absent");
    let rows = detect(&["flake.nix"], None);
    let path = seed_init(&dir.0, CONFIG, &rows).ok();
    assert_eq!(path, Some(dir.0.join(CONFIG)));
    let written = fs::read_to_string(dir.0.join(CONFIG)).unwrap_or_default();
    assert_eq!(written, text(&rows));
}

#[test]
fn the_caller_names_the_file() {
    let dir = Scratch::new("named-file");
    let path = seed_init(&dir.0, ".actlith", &[]).ok();
    assert_eq!(path, Some(dir.0.join(".actlith")));
    assert!(!dir.0.join(CONFIG).exists());
}

/// Seed over a hand-written config; returns the error, the path and the
/// file as it is afterwards.
fn seed_over_existing(name: &str) -> (Option<SeedError>, PathBuf, String) {
    let dir = Scratch::new(name);
    let path = dir.0.join(CONFIG);
    assert!(fs::write(&path, "format 1\n# mine\n").is_ok());
    let err = seed_init(&dir.0, CONFIG, &detect(&["flake.nix"], None)).err();
    let kept = fs::read_to_string(&path).unwrap_or_default();
    (err, path, kept)
}

#[test]
fn an_existing_file_is_refused_and_kept() {
    let (err, path, kept) = seed_over_existing("exists");
    assert!(matches!(&err, Some(SeedError::Exists(p)) if *p == path));
    assert_eq!(err.as_ref().map(SeedError::code), Some(1));
    assert_eq!(kept, "format 1\n# mine\n");
}

#[test]
fn the_refusal_names_the_file() {
    let (err, path, _) = seed_over_existing("named");
    let shown = err.map(|e| e.to_string()).unwrap_or_default();
    assert!(shown.contains(&path.display().to_string()), "{shown}");
    assert!(shown.contains("already exists"), "{shown}");
}

#[test]
fn an_unwritable_root_is_a_write_error() {
    let dir = Scratch::new("missing");
    let err = seed_init(&dir.0.join("no-such-dir"), CONFIG, &[]).err();
    assert!(
        matches!(&err, Some(SeedError::Write(p, _)) if p.ends_with(CONFIG))
    );
    assert_eq!(err.as_ref().map(SeedError::code), Some(2));
    let shown = err.map(|e| e.to_string()).unwrap_or_default();
    assert!(shown.contains("cannot write"), "{shown}");
}
