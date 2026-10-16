//! The facts the README badges and the LLM disclaimer quote, each read from
//! the file that owns it. Pure functions over text: the caller reads the
//! repository and hands the text in, so every rule is testable without one.
//! Ported from actlith-dev, itself from pklith-dev; MIT, same author.

/// A `key = "value"` line's value, from a TOML file read as text.
#[must_use]
pub(crate) fn quoted(toml: &str, key: &str) -> Option<String> {
    let prefix = format!("{key} = \"");
    toml.lines()
        .find_map(|l| l.strip_prefix(&prefix)?.strip_suffix('"'))
        .map(str::to_owned)
}

/// How many entries the `[dependencies]` table of a Cargo.toml holds;
/// `None` when it has no such table, so zero is a count, never a miss.
#[must_use]
pub(crate) fn dependencies(toml: &str) -> Option<usize> {
    let mut lines = toml.lines().skip_while(|l| l.trim() != "[dependencies]");
    lines.next()?;
    let entries = lines
        .take_while(|l| !l.starts_with('['))
        .filter(|l| l.split('=').next().is_some_and(is_key));
    Some(entries.count())
}

fn is_key(s: &str) -> bool {
    let key = s.trim();
    !key.is_empty()
        && !key.starts_with('#')
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
}

/// The coverage floor `.coverage` records, trailing zeros dropped:
/// `lines 100.00` is `100`, `lines 92.30` is `92.3`.
#[must_use]
pub(crate) fn floor(coverage: &str) -> Option<String> {
    let value = coverage.lines().find_map(|l| l.strip_prefix("lines "))?;
    let value = value.trim();
    let trimmed = if value.contains('.') {
        value.trim_end_matches('0').trim_end_matches('.')
    } else {
        value
    };
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

/// Rows of a spec table across `specs`: lines starting with `prefix`,
/// digits, then `sep`. Every node numbers its own rows from 1, so rows are
/// counted, never distinct ids (pklith dev B1).
#[must_use]
pub(crate) fn rows(specs: &[String], prefix: char, sep: char) -> usize {
    let row = |l: &&str| {
        l.strip_prefix(prefix).is_some_and(|rest| {
            let digits = rest.trim_start_matches(|c: char| c.is_ascii_digit());
            digits.len() < rest.len() && digits.starts_with(sep)
        })
    };
    specs.iter().map(|s| s.lines().filter(row).count()).sum()
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

/// The `nixpkgs` node of a flake lock. Its header is `"nixpkgs": {`; an
/// input that follows it is `"nixpkgs": [`, which a line match on the name
/// alone would enter by mistake (sherd dev B1).
#[must_use]
pub(crate) fn nixpkgs(lock: &str) -> Option<Nixpkgs> {
    let node: Vec<&str> = lock
        .lines()
        .skip_while(|l| l.trim() != "\"nixpkgs\": {")
        .take_while(|l| {
            !l.trim_start().starts_with('}') || l.starts_with("      ")
        })
        .collect();
    let field = |name: &str| field(&node, name);
    let series = field("ref")?.strip_prefix("nixos-")?.to_owned();
    let rev = field("rev")?.get(..7)?.to_owned();
    let day = day(field("lastModified")?.parse().ok()?)?;
    Some(Nixpkgs { series, rev, day })
}

/// A `"name": value` field's value among `node`'s lines, unquoted.
fn field(node: &[&str], name: &str) -> Option<String> {
    let key = format!("\"{name}\":");
    node.iter()
        .find_map(|l| l.trim().strip_prefix(&key))
        .map(|v| v.trim().trim_end_matches(',').trim_matches('"').to_owned())
}

/// `YYYY-MM-DD` for a Unix time, in UTC; `None` before 1970.
#[must_use]
pub(crate) fn day(secs: i64) -> Option<String> {
    let mut days = u64::try_from(secs).ok()?.checked_div(86_400)?;
    let mut year: u64 = 1970;
    while days >= year_days(year) {
        days = days.checked_sub(year_days(year))?;
        year = year.checked_add(1)?;
    }
    let (month, day) = month_day(year, days)?;
    Some(format!("{year:04}-{month:02}-{day:02}"))
}

fn leap(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100))
        || year.is_multiple_of(400)
}

fn year_days(year: u64) -> u64 {
    if leap(year) { 366 } else { 365 }
}

/// Month and day of month for day `days` (from 0) of `year`.
fn month_day(year: u64, mut days: u64) -> Option<(u64, u64)> {
    let feb = if leap(year) { 29 } else { 28 };
    let months = [31, feb, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (month, len) in (1..).zip(months) {
        if days < len {
            return Some((month, days.checked_add(1)?));
        }
        days = days.checked_sub(len)?;
    }
    None
}

/// A platform badge: architecture and operating system.
pub(crate) type Platform = (&'static str, &'static str);

/// The platforms CI's matrix builds on, in badge order. Not the flake's
/// systems: a system no runner builds is not a claim CI backs.
///
/// # Errors
///
/// A runner this table does not know, named: guessing its platform would
/// badge something nothing checked.
pub(crate) fn platforms(ci: &str) -> Result<Vec<Platform>, String> {
    let list = ci
        .lines()
        .find_map(|l| l.trim().strip_prefix("os: ["))
        .and_then(|l| l.strip_suffix(']'))
        .ok_or("ci.yml has no `os: [...]` matrix")?;
    let mut found = Vec::new();
    for os in list.split(',').map(str::trim) {
        let why = format!("ci.yml runner `{os}` has no known platform");
        found.extend(runner(os).ok_or(why)?);
    }
    Ok(ORDER.into_iter().filter(|p| found.contains(p)).collect())
}

/// Badge order, whatever order the matrix lists runners in.
const ORDER: [Platform; 5] = [
    ("intel", "linux"),
    ("amd", "linux"),
    ("arm", "linux"),
    ("intel", "macos"),
    ("arm", "macos"),
];

fn runner(os: &str) -> Option<Vec<Platform>> {
    match os {
        o if o.starts_with("ubuntu") && o.ends_with("-arm") => {
            Some(vec![("arm", "linux")])
        }
        o if o.starts_with("ubuntu") => {
            Some(vec![("intel", "linux"), ("amd", "linux")])
        }
        "macos-13" => Some(vec![("intel", "macos")]),
        o if o.starts_with("macos") => Some(vec![("arm", "macos")]),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
