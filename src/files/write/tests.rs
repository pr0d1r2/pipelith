//! `write` over a temp root: writes listed paths only, only on change
//! (V29, V30).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::write;
use crate::files::File;

static NEXT: AtomicUsize = AtomicUsize::new(0);

/// A fresh, empty directory for one test, removed when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let id = std::process::id();
        let leaf = format!("pipelith-files-write-{id}-{n}-{name}");
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

fn two() -> Vec<File> {
    vec![
        file("ci/pipelines/build.yml", "name: ci\n"),
        file("scripts/tool/open-pr.sh", "echo pr\n"),
    ]
}

fn read(root: &Path, rel: &str) -> String {
    fs::read_to_string(root.join(rel)).unwrap_or_default()
}

#[test]
fn writes_every_file_creating_its_parents() {
    let root = Scratch::new("fresh");
    let written = write(&root.0, &two()).ok();
    let want = ["ci/pipelines/build.yml", "scripts/tool/open-pr.sh"];
    assert_eq!(written, Some(want.map(str::to_owned).to_vec()));
    assert_eq!(read(&root.0, "ci/pipelines/build.yml"), "name: ci\n");
    assert_eq!(read(&root.0, "scripts/tool/open-pr.sh"), "echo pr\n");
}

#[test]
fn writes_nothing_when_every_file_is_current() {
    let root = Scratch::new("current");
    assert!(write(&root.0, &two()).is_ok());
    assert_eq!(write(&root.0, &two()).ok(), Some(Vec::new()));
}

#[test]
fn keeps_the_mtime_of_a_current_file() {
    let root = Scratch::new("mtime");
    assert!(write(&root.0, &two()).is_ok());
    let path = root.0.join("ci/pipelines/build.yml");
    let old = filetime(&path, 1_000_000_000);
    assert!(old.is_some());
    assert!(write(&root.0, &two()).is_ok());
    let now = fs::metadata(&path).and_then(|m| m.modified()).ok();
    assert_eq!(now, old);
}

/// Set the mtime of `path` to `secs` after the epoch; the time it now has.
fn filetime(path: &Path, secs: u64) -> Option<std::time::SystemTime> {
    let when = std::time::UNIX_EPOCH
        .checked_add(std::time::Duration::from_secs(secs))?;
    let handle = fs::File::options().write(true).open(path).ok()?;
    handle.set_modified(when).ok()?;
    fs::metadata(path).and_then(|m| m.modified()).ok()
}

#[test]
fn rewrites_only_the_file_whose_bytes_differ() {
    let root = Scratch::new("changed");
    assert!(write(&root.0, &two()).is_ok());
    let ci = root.0.join("ci/pipelines/build.yml");
    assert!(fs::write(&ci, "hand edit\n").is_ok());
    let written = write(&root.0, &two()).ok();
    assert_eq!(written, Some(vec!["ci/pipelines/build.yml".to_owned()]));
    assert_eq!(read(&root.0, "ci/pipelines/build.yml"), "name: ci\n");
}

#[test]
fn leaves_unlisted_files_alone() {
    let root = Scratch::new("unlisted");
    assert!(fs::create_dir_all(root.0.join("ci/pipelines")).is_ok());
    let mine = root.0.join("ci/pipelines/mine.yml");
    assert!(fs::write(&mine, "hand made\n").is_ok());
    assert!(fs::write(root.0.join("README"), "keep\n").is_ok());
    assert!(write(&root.0, &two()).is_ok());
    assert_eq!(read(&root.0, "ci/pipelines/mine.yml"), "hand made\n");
    assert_eq!(read(&root.0, "README"), "keep\n");
}

#[test]
fn refuses_an_escaping_path_before_writing_anything() {
    let root = Scratch::new("escape");
    let mut files = two();
    files.push(file("../outside.yml", "x\n"));
    assert!(write(&root.0.join("in"), &files).is_err());
    assert!(!root.0.join("outside.yml").exists());
    assert!(!root.0.join("in").exists());
}

#[test]
fn reports_a_directory_where_a_file_goes() {
    let root = Scratch::new("dir");
    let at = root.0.join("ci/pipelines/build.yml");
    assert!(fs::create_dir_all(at).is_ok());
    assert!(write(&root.0, &two()).is_err());
}

#[test]
fn reports_a_failed_parent_directory() {
    let root = Scratch::new("blocked");
    assert!(fs::write(root.0.join("ci"), "a file, not a dir\n").is_ok());
    assert!(write(&root.0, &two()).is_err());
}
