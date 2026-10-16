//! Argument dispatch for `pipelith-dev`.
//!
//! ```text
//! pipelith-dev readme [--check]    README badges and the disclaimer's numbers
//! pipelith-dev notices [--check]   the third-party notices' crate table
//! pipelith-dev agents [--check]    the step table in AGENTS.md
//! pipelith-dev commit-msg [FILE]   the commit message style rule
//! pipelith-dev changelog [FILE]    a behaviour change adds an entry
//! ```
//!
//! Exit 0 clean, 1 drift or a broken rule, 2 usage or I/O.

use crate::hooks;
use crate::io::Failed;
use std::path::Path;

const USAGE: &str = "usage: pipelith-dev <readme|notices|agents> [--check] | <commit-msg|changelog> [FILE]";

/// Run one verb against the repository at `repo`; the exit code.
#[must_use]
pub fn run(args: &[String], repo: &Path) -> u8 {
    match dispatch(args, repo) {
        Ok(()) => 0,
        Err((code, why)) => {
            eprintln!("pipelith-dev: {why}");
            code
        }
    }
}

fn dispatch(args: &[String], repo: &Path) -> Result<(), Failed> {
    let words: Vec<&str> = args.iter().map(String::as_str).collect();
    match words.as_slice() {
        ["commit-msg", rest @ ..] if rest.len() < 2 => {
            hooks::commit_msg(repo, rest.first().copied())
        }
        ["changelog", rest @ ..] if rest.len() < 2 => {
            hooks::changelog(repo, rest.first().copied())
        }
        [verb] => generated(repo, verb, false),
        [verb, "--check"] => generated(repo, verb, true),
        _ => Err((2, USAGE.to_owned())),
    }
}

/// A generated file, written or checked.
fn generated(repo: &Path, verb: &str, check: bool) -> Result<(), Failed> {
    match verb {
        "readme" => crate::readme::run_verb(repo, check),
        "notices" => crate::notices::run_verb(repo, check),
        "agents" => crate::agents::run_verb(repo, check),
        _ => Err((2, USAGE.to_owned())),
    }
}

#[cfg(test)]
mod tests;
