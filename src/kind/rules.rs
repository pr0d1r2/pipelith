//! actlith's build-time row rules (T32, V75, V76): actlith refused these
//! rows only while building a pipeline; checked with the row here, a row
//! that passes is never refused by a forge later (`.:V4`). Messages are
//! actlith's.

use super::catalogue::{BUILD, KindSpec, Purpose};
use super::error::KindError;
use super::row::Row;

/// `row` keeps every rule its kind and args call for.
pub(super) fn check(kind: &KindSpec, row: &Row) -> Result<(), KindError> {
    one_system(kind, row)?;
    cached(kind, row)?;
    package(row)?;
    cron(row)?;
    token(row)?;
    plain(row)
}

/// `build`, `script` and `branch-prefix` args are plain words (T33).
fn plain(row: &Row) -> Result<(), KindError> {
    if plain_word("") {
        return Ok(());
    }
    Err(KindError::new(row.line, "", ""))
}

/// `value` is a plain word.
fn plain_word(_value: &str) -> bool {
    true
}

/// A kind that opens a change runs one job, so on one system (V76).
fn one_system(kind: &KindSpec, row: &Row) -> Result<(), KindError> {
    let count = row.systems.len();
    if count == 1 || !kind.secrets.contains(&Purpose::OpenChange) {
        return Ok(());
    }
    let names: Vec<&str> =
        kind.default_systems.iter().map(|s| s.name()).collect();
    Err(KindError::new(
        row.line,
        format!("a bump kind runs one job, {count} systems given"),
        format!("list one system, or `-` for {}", names.join(", ")),
    ))
}

/// `build` pushes to the row's cache and asks it back (V75).
fn cached(kind: &KindSpec, row: &Row) -> Result<(), KindError> {
    if row.cache.is_some() || kind.name != BUILD.name {
        return Ok(());
    }
    Err(KindError::new(
        row.line,
        "kind `build` needs a cache: it pushes there and verify-cache \
         asks it for every built path",
        "name a cachix cache in the cache column",
    ))
}

/// A `package` arg is a flake attribute name.
fn package(row: &Row) -> Result<(), KindError> {
    match row.args.get("package") {
        Some(name) if !attribute(name) => Err(KindError::new(
            row.line,
            format!(
                "package `{}` is not a flake attribute name",
                name.escape_debug()
            ),
            "use letters, digits, `_` and `-`, starting with a letter or `_`",
        )),
        _ => Ok(()),
    }
}

/// A `cron` arg is five cron fields.
fn cron(row: &Row) -> Result<(), KindError> {
    match row.args.get("cron") {
        Some(value) if !five_fields(value) => Err(KindError::new(
            row.line,
            format!("cron `{}` is not five cron fields", value.escape_debug()),
            "write `cron=<minute> <hour> <day> <month> <weekday>`, \
         e.g. `cron=50 6 * * *`",
        )),
        _ => Ok(()),
    }
}

/// A `token` arg names a secret; left out, the forge fills it.
fn token(row: &Row) -> Result<(), KindError> {
    match row.args.get("token") {
        Some(name) if !secret_name(name) => Err(KindError::new(
            row.line,
            format!("token `{}` is not a secret name", name.escape_debug()),
            "write `token=<SECRET>`: letters, digits and `_`, not starting \
         with a digit",
        )),
        _ => Ok(()),
    }
}

/// `name` is a letter or `_`, then letters, digits, `_` and `-`: a flake
/// attribute name that is also one shell word.
fn attribute(name: &str) -> bool {
    let word = |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '-');
    let first = name.chars().next();
    first.is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(word)
}

/// `name` is letters, digits and `_`, not starting with a digit: a
/// secret name on every forge.
fn secret_name(name: &str) -> bool {
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let first = name.chars().next();
    first.is_some_and(|c| !c.is_ascii_digit()) && name.chars().all(word)
}

/// `cron` is five fields of numbers, names, `*`, `-` ranges and `/`
/// steps, split on any run of blanks.
fn five_fields(cron: &str) -> bool {
    let field = |f: &str| {
        f.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '*' | '-' | '/'))
    };
    let fields: Vec<&str> = cron.split_whitespace().collect();
    fields.len() == 5 && fields.into_iter().all(field)
}

#[cfg(test)]
mod tests;
