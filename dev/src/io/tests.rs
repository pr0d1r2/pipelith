use super::{hooks, message, read, run, show, steps, update};
use std::path::{Path, PathBuf};
use std::process::Command;

/// A fresh git repository in the system temp directory.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("pipelith-dev-io-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::create_dir_all(&dir);
    git(&dir, &["init", "-q"]);
    dir
}

fn git(repo: &Path, args: &[&str]) {
    let mut command = Command::new("git");
    for var in crate::io::GIT_REPO_VARS {
        command.env_remove(var);
    }
    let _ = command
        .args(["-c", "user.name=t", "-c", "user.email=t@t"])
        .args(["-c", "core.hooksPath=/dev/null"])
        .args(args)
        .current_dir(repo)
        .output();
}

#[test]
fn run_returns_stdout_in_the_repository() {
    let repo = scratch("run-ok");
    let out = run(&repo, "git", &["rev-parse", "--is-inside-work-tree"]);
    assert_eq!(out, Ok("true\n".to_owned()));
}

#[test]
fn run_reports_a_failing_or_missing_program() {
    let repo = scratch("run-bad");
    let failed = run(&repo, "git", &["no-such-verb"]).err();
    let (code, why) = failed.unwrap_or_default();
    assert_eq!(code, 2);
    assert!(why.starts_with("git no-such-verb failed:"), "{why}");
    let missing = run(&repo, "pipelith-no-such-program", &[]).err();
    let (code, why) = missing.unwrap_or_default();
    assert_eq!(code, 2);
    assert!(
        why.starts_with("cannot run pipelith-no-such-program"),
        "{why}"
    );
}

#[test]
fn show_is_empty_for_a_missing_object() {
    let repo = scratch("show");
    assert_eq!(show(&repo, "HEAD~1:CHANGELOG.md"), Ok(String::new()));
    let _ = std::fs::write(repo.join("f"), "x\n");
    git(&repo, &["add", "f"]);
    git(&repo, &["commit", "-qm", "c"]);
    assert_eq!(show(&repo, "HEAD:f"), Ok("x\n".to_owned()));
}

#[test]
fn message_reads_a_file_or_commit_editmsg() {
    let repo = scratch("message");
    let _ = std::fs::write(repo.join("m"), "feat: a\n");
    assert_eq!(message(&repo, Some("m")), Ok("feat: a\n".to_owned()));
    let _ = std::fs::write(repo.join(".git/COMMIT_EDITMSG"), "fix: b\n");
    assert_eq!(message(&repo, None), Ok("fix: b\n".to_owned()));
    let (code, why) = message(&repo, Some("gone")).err().unwrap_or_default();
    assert_eq!(code, 2);
    assert!(why.starts_with("no message file at"), "{why}");
}

#[test]
fn a_scratch_repository_call_drops_the_hook_repository_variables() {
    let cmd = super::command(Path::new("/tmp/elsewhere"), "git", &["status"]);
    let removed: Vec<_> = cmd
        .get_envs()
        .filter(|(_, value)| value.is_none())
        .map(|(key, _)| key.to_string_lossy().into_owned())
        .collect();
    for var in super::GIT_REPO_VARS {
        assert!(removed.iter().any(|k| k == var), "{var} kept");
    }
}

#[test]
fn a_call_in_the_hooked_repository_keeps_its_variables() {
    let cmd = super::command(Path::new("."), "git", &["status"]);
    assert_eq!(cmd.get_envs().count(), 0);
}

const DOC: super::Doc = super::Doc {
    path: "f.md",
    verb: "v",
};

#[test]
fn read_names_an_unreadable_path() {
    let repo = scratch("read");
    let _ = std::fs::write(repo.join("f.md"), "a\n");
    assert_eq!(read(&repo, "f.md"), Ok("a\n".to_owned()));
    let (code, why) = read(&repo, "gone").err().unwrap_or_default();
    assert!(code == 2 && why.starts_with("cannot read gone"), "{why}");
}

/// Equal is clean, `check` names the fix, otherwise the file is written.
#[test]
fn update_writes_or_reports_drift() {
    let repo = scratch("update");
    let _ = std::fs::write(repo.join("f.md"), "old\n");
    assert_eq!(update(&repo, &DOC, "old\n", true), Ok(()));
    let (code, why) =
        update(&repo, &DOC, "new\n", true).err().unwrap_or_default();
    assert_eq!(code, 1);
    assert_eq!(
        why,
        "f.md drifted from its sources; run `cargo run -p pipelith-dev -- v`"
    );
    assert_eq!(update(&repo, &DOC, "new\n", false), Ok(()));
    assert_eq!(read(&repo, "f.md"), Ok("new\n".to_owned()));
}

/// hk.pkl's hooks counted by `pkl`; text that is not a number is exit 2.
#[test]
fn steps_are_counted_by_pkl() {
    let repo = scratch("steps");
    let hk = "hooks {\n  [\"pre-commit\"] { steps { length = 3 } }\n  [\"pre-push\"] { steps { length = \"x\" } }\n}\n";
    let _ = std::fs::write(repo.join("hk.pkl"), hk);
    assert_eq!(steps(&repo, "pre-commit"), Ok(3));
    let (code, why) = hooks(&repo).err().unwrap_or_default();
    assert_eq!(code, 2);
    assert!(why.starts_with("pkl printed `x`"), "{why}");
}
