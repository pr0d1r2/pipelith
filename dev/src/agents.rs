//! `pipelith-dev agents [--check]`: the step table in AGENTS.md, from the
//! steps hk.pkl gives each hook (dev T18, V40). Ported from actlith-dev,
//! itself from pklith-dev; MIT, same author.

use crate::block;
use crate::io::{Doc, Failed, hooks, read, run, update};
use std::fmt::Write as _;
use std::path::Path;

const AGENTS: Doc = Doc {
    path: "AGENTS.md",
    verb: "agents",
};

/// Every push step as one line: id, `commit` or `push`, globs joined by
/// U+001F, and whether it fixes; tab-separated. Asking `pkl` for text
/// keeps the dev crate free of a JSON parser.
const ROWS: &str = r#"let (fast = hooks["pre-commit"].steps) hooks["pre-push"].steps.toMap().entries.map((e) -> let (s = e.value) List(e.key, if (fast.containsKey(e.key)) "commit" else "push", (if (s.glob is String) List(s.glob) else s.glob ?? List()).join("\u{1F}"), if (s.fix != null) "yes" else "-").join("\t")).join("\n")"#;

/// The table block, markers included, for `rows` (as `ROWS` prints them)
/// and the number of commit and push steps.
#[must_use]
pub(crate) fn table(rows: &str, commit: usize, push: usize) -> String {
    let mut out = format!(
        "<!-- BEGIN steps: generated from hk.pkl by `cargo run -p pipelith-dev -- agents`; do not edit -->\n{commit} steps run on every commit and {push} on push and `hk check`; the commit-msg hook checks the message.\n\n| step | runs on | files | fixes |\n| --- | --- | --- | --- |\n"
    );
    for line in rows.lines().filter(|l| !l.is_empty()) {
        let mut cells = line.split('\t');
        let mut next = || cells.next().unwrap_or_default();
        let (id, when, globs, fix) = (next(), next(), next(), next());
        let _ = writeln!(out, "| `{id}` | {when} | {} | {fix} |", files(globs));
    }
    out.push_str("<!-- END steps -->\n");
    out
}

/// Globs as code spans, or "whole tree" for a step without one.
fn files(globs: &str) -> String {
    if globs.is_empty() {
        return "whole tree".to_owned();
    }
    let spans: Vec<String> =
        globs.split('\u{1F}').map(|g| format!("`{g}`")).collect();
    spans.join(" ")
}

/// Render or check.
///
/// # Errors
///
/// Drift (1), `pkl` or a file failing (2).
pub(crate) fn run_verb(repo: &Path, check: bool) -> Result<(), Failed> {
    let rows = run(repo, "pkl", &["eval", "-x", ROWS, "hk.pkl"])?;
    let (commit, push) = hooks(repo)?;
    let doc = read(repo, AGENTS.path)?;
    let wanted = block::splice(&doc, "steps", &table(&rows, commit, push))
        .ok_or((2, "AGENTS.md has no <!-- BEGIN steps --> block".to_owned()))?;
    update(repo, &AGENTS, &wanted, check)
}

#[cfg(test)]
mod tests;
