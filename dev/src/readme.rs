//! `pipelith-dev readme [--check]`: the README badge block, and the numbers
//! `docs/LLM-DISCLAIMER.md` quotes, from the files that own them (dev T18,
//! V40).

use crate::badges::{self, Cargo, Facts};
use crate::block;
use crate::facts::{self, Nixpkgs, Platform};
use crate::io::{Doc, Failed, hooks, read, run, update};
use std::path::Path;

const README: Doc = Doc {
    path: "README.md",
    verb: "readme",
};

const DISCLAIMER: &str = "docs/LLM-DISCLAIMER.md";

/// Render or check.
///
/// # Errors
///
/// Stale disclaimer prose or drift (1), an unreadable source (2).
pub(crate) fn run_verb(repo: &Path, check: bool) -> Result<(), Failed> {
    let facts = gather(repo)?.checked().map_err(|e| (2, e))?;
    let stale = badges::stale_prose(&read(repo, DISCLAIMER)?, &facts);
    if let Some(want) = stale.first() {
        let why = format!("{DISCLAIMER} must say {want}, as its sources do");
        return Err((1, why));
    }
    let doc = read(repo, README.path)?;
    let wanted = block::splice(&doc, "badges", &badges::render(&facts)).ok_or(
        (2, "README.md has no <!-- BEGIN badges --> block".to_owned()),
    )?;
    update(repo, &README, &wanted, check)
}

fn gather(repo: &Path) -> Result<Facts, Failed> {
    let (floor, nixpkgs, platforms) = files(repo)?;
    Ok(Facts {
        cargo: cargo(&read(repo, "Cargo.toml")?)?,
        floor,
        steps: hooks(repo)?,
        spec: spec(repo)?,
        nixpkgs,
        platforms,
    })
}

/// The facts Cargo.toml owns.
fn cargo(text: &str) -> Result<Cargo, Failed> {
    let field = |key: &str| facts::quoted(text, key).unwrap_or_default();
    let no_table = || (2, "Cargo.toml has no [dependencies] table".to_owned());
    Ok(Cargo {
        slug: field("repository").replace("https://github.com/", ""),
        license: field("license"),
        edition: field("edition"),
        msrv: field("rust-version"),
        dependencies: facts::dependencies(text).ok_or_else(no_table)?,
    })
}

/// The facts `.coverage`, `flake.lock` and CI's workflow own.
fn files(repo: &Path) -> Result<(String, Nixpkgs, Vec<Platform>), Failed> {
    let missing = |what: &str| (2, format!("{what} holds no readable value"));
    let floor = facts::floor(&read(repo, ".coverage")?)
        .ok_or_else(|| missing(".coverage"))?;
    let nixpkgs = facts::nixpkgs(&read(repo, "flake.lock")?)
        .ok_or_else(|| missing("flake.lock nixpkgs"))?;
    let ci = read(repo, ".github/workflows/ci.yml")?;
    let platforms = facts::platforms(&ci).map_err(|e| (2, e))?;
    Ok((floor, nixpkgs, platforms))
}

/// `V` rows, `B` rows and nodes, over every tracked SPEC.md.
fn spec(repo: &Path) -> Result<(usize, usize, usize), Failed> {
    let listed = run(repo, "git", &["ls-files", "-z", "*SPEC.md"])?;
    let texts = listed
        .split('\0')
        .filter(|p| !p.is_empty() && repo.join(p).is_file())
        .map(|p| read(repo, p))
        .collect::<Result<Vec<_>, _>>()?;
    let rows = |prefix, sep| facts::rows(&texts, prefix, sep);
    Ok((rows('V', ':'), rows('B', '|'), texts.len()))
}

#[cfg(test)]
mod tests;
