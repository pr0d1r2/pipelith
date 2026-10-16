//! The only module that touches the repository: git, `pkl`, `cargo` and
//! files, always
//! relative to an explicit repository directory so tests can use scratch
//! repositories.

use std::path::Path;
use std::process::Command;

/// An exit code and the reason: 1 a broken rule, 2 usage or I/O.
pub(crate) type Failed = (u8, String);

/// Run a program in `repo`; its stdout on success.
///
/// # Errors
///
/// (2, why) when it cannot start or exits non-zero.
pub(crate) fn run(
    repo: &Path,
    program: &str,
    args: &[&str],
) -> Result<String, Failed> {
    let _ = (repo, program, args);
    Ok(String::new())
}

/// The variables git exports to hooks that pin a command to one
/// repository; a call aimed at another directory must drop them
/// (actlith nix:B4; nix V47).
pub(crate) const GIT_REPO_VARS: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

/// The command for `program` in `repo`, with the repository variables
/// dropped unless `repo` is the current directory.
pub(crate) fn command(repo: &Path, program: &str, args: &[&str]) -> Command {
    let _ = (repo, args);
    Command::new(program)
}

/// `git show OBJECT` in `repo`, or empty text when git has no such object.
///
/// # Errors
///
/// (2, why) when git cannot start.
pub(crate) fn show(repo: &Path, object: &str) -> Result<String, Failed> {
    let _ = (repo, object);
    Err((2, String::new()))
}

/// The commit message: FILE relative to `repo`, or git's COMMIT_EDITMSG.
///
/// # Errors
///
/// (2, why) when there is no message to judge.
pub(crate) fn message(
    repo: &Path,
    file: Option<&str>,
) -> Result<String, Failed> {
    let _ = (repo, file);
    Ok(String::new())
}

/// A generated file and the verb that writes it.
pub(crate) struct Doc {
    /// The path, relative to the repository.
    pub(crate) path: &'static str,
    /// The `pipelith-dev` verb that renders it.
    pub(crate) verb: &'static str,
}

/// A file's text, `path` relative to `repo`.
///
/// # Errors
///
/// (2, why) naming the path when it cannot be read.
pub(crate) fn read(repo: &Path, path: &str) -> Result<String, Failed> {
    let _ = (repo, path);
    Ok(String::new())
}

/// Make `doc` hold `wanted`: written, or with `check` compared.
///
/// # Errors
///
/// (1, why) on drift under `check`; (2, why) when it cannot be read or
/// written.
pub(crate) fn update(
    repo: &Path,
    doc: &Doc,
    wanted: &str,
    check: bool,
) -> Result<(), Failed> {
    let _ = (repo, doc, wanted, check);
    Ok(())
}

/// Steps in one hook, as `pkl` evaluates `hk.pkl` in `repo`.
///
/// # Errors
///
/// (2, why) when `pkl` fails or prints something other than a number.
pub(crate) fn steps(repo: &Path, hook: &str) -> Result<usize, Failed> {
    let _ = (repo, hook);
    Ok(0)
}

/// Steps on commit and on push.
///
/// # Errors
///
/// As [`steps`].
pub(crate) fn hooks(repo: &Path) -> Result<(usize, usize), Failed> {
    let _ = repo;
    Ok((0, 0))
}

#[cfg(test)]
mod tests;
