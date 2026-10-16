//! The facts the README badges and the LLM disclaimer quote, each read from
//! the file that owns it. Pure functions over text: the caller reads the
//! repository and hands the text in, so every rule is testable without one.
//! Ported from actlith-dev, itself from pklith-dev; MIT, same author.

/// A `key = "value"` line's value, from a TOML file read as text.
#[must_use]
pub(crate) fn quoted(toml: &str, key: &str) -> Option<String> {
    let _ = (toml, key);
    None
}

/// How many entries the `[dependencies]` table of a Cargo.toml holds;
/// `None` when it has no such table, so zero is a count, never a miss.
#[must_use]
pub(crate) fn dependencies(toml: &str) -> Option<usize> {
    let _ = toml;
    None
}

/// The coverage floor `.coverage` records, trailing zeros dropped.
#[must_use]
pub(crate) fn floor(coverage: &str) -> Option<String> {
    let _ = coverage;
    None
}

/// Rows of a spec table across `specs`: lines starting with `prefix`,
/// digits, then `sep`.
#[must_use]
pub(crate) fn rows(specs: &[String], prefix: char, sep: char) -> usize {
    let _ = (specs, prefix, sep);
    0
}

/// The nixpkgs the flake builds with, from `flake.lock`.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Nixpkgs {
    /// The release series, `26.05` from `nixos-26.05`.
    pub(crate) series: String,
    /// The locked commit, 7 characters.
    pub(crate) rev: String,
    /// The commit date, `YYYY-MM-DD`.
    pub(crate) day: String,
}

/// The `nixpkgs` node of a flake lock.
#[must_use]
pub(crate) fn nixpkgs(lock: &str) -> Option<Nixpkgs> {
    let _ = lock;
    None
}

/// `YYYY-MM-DD` for a Unix time, in UTC; `None` before 1970.
#[must_use]
pub(crate) fn day(secs: i64) -> Option<String> {
    let _ = secs;
    None
}

/// Month and day of month for day `days` (from 0) of `year`.
fn month_day(year: u64, days: u64) -> Option<(u64, u64)> {
    let _ = (year, days);
    None
}

/// A platform badge: architecture and operating system.
pub(crate) type Platform = (&'static str, &'static str);

/// The platforms CI's matrix builds on, in badge order.
///
/// # Errors
///
/// A runner this table does not know, named.
pub(crate) fn platforms(ci: &str) -> Result<Vec<Platform>, String> {
    let _ = ci;
    Ok(Vec::new())
}

#[cfg(test)]
mod tests;
