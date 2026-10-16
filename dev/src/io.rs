//! The only module that touches the repository: git, `pkl`, `cargo` and
//! files, always
//! relative to an explicit repository directory so tests can use scratch
//! repositories.

use std::path::Path;
use std::process::{Command, Output};

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
    let out = spawn(repo, program, args)?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        let why =
            format!("{program} {} failed: {}", args.join(" "), err.trim());
        return Err((2, why));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The variables git exports to hooks that pin a command to one
/// repository. A hook runs this crate with them set; a call aimed at
/// another directory (a scratch repository in tests) must drop them,
/// or git acts on the hooked repository instead and re-enters its
/// hooks, which forked without bound once (actlith nix:B4; nix V47).
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
    let mut command = Command::new(program);
    command.args(args).current_dir(repo);
    if repo != Path::new(".") {
        for var in GIT_REPO_VARS {
            command.env_remove(var);
        }
    }
    command
}

fn spawn(repo: &Path, program: &str, args: &[&str]) -> Result<Output, Failed> {
    command(repo, program, args)
        .output()
        .map_err(|e| (2, format!("cannot run {program}: {e}")))
}

/// `git show OBJECT` in `repo`, or empty text when git has no such object
/// (a first commit has no `HEAD~1`).
///
/// # Errors
///
/// (2, why) when git cannot start.
pub(crate) fn show(repo: &Path, object: &str) -> Result<String, Failed> {
    let out = spawn(repo, "git", &["show", object])?;
    if !out.status.success() {
        return Ok(String::new());
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
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
    let path = match file {
        Some(f) => f.to_owned(),
        None => {
            run(repo, "git", &["rev-parse", "--git-path", "COMMIT_EDITMSG"])?
                .trim()
                .to_owned()
        }
    };
    std::fs::read_to_string(repo.join(&path)).map_err(|_| {
        (
            2,
            format!("no message file at {path}, so nothing was checked"),
        )
    })
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
    std::fs::read_to_string(repo.join(path))
        .map_err(|e| (2, format!("cannot read {path}: {e}")))
}

/// Make `doc` hold `wanted`: written, or with `check` compared, drift
/// being exit 1 naming the verb that fixes it (equality, never a
/// heuristic).
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
    let (path, verb) = (doc.path, doc.verb);
    if read(repo, path)? == wanted {
        return Ok(());
    }
    if check {
        let fix = format!(
            "{path} drifted from its sources; run `cargo run -p pipelith-dev -- {verb}`"
        );
        return Err((1, fix));
    }
    std::fs::write(repo.join(path), wanted)
        .map_err(|e| (2, format!("cannot write {path}: {e}")))
}

/// Steps in one hook, as `pkl` evaluates `hk.pkl` in `repo`.
///
/// # Errors
///
/// (2, why) when `pkl` fails or prints something other than a number.
pub(crate) fn steps(repo: &Path, hook: &str) -> Result<usize, Failed> {
    let expr = format!("hooks[\"{hook}\"].steps.length");
    let text = run(repo, "pkl", &["eval", "-x", &expr, "hk.pkl"])?;
    text.trim()
        .parse()
        .map_err(|_| (2, format!("pkl printed `{}` for {expr}", text.trim())))
}

/// Steps on commit and on push.
///
/// # Errors
///
/// As [`steps`].
pub(crate) fn hooks(repo: &Path) -> Result<(usize, usize), Failed> {
    Ok((steps(repo, "pre-commit")?, steps(repo, "pre-push")?))
}

#[cfg(test)]
mod tests;
