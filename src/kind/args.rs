//! A row's `args` checked against its kind's schema plus the forge's
//! extra keys (V31): an unknown, repeated or missing key is an error
//! naming the row's line and the key, never ignored or silently
//! defaulted.

use std::collections::BTreeMap;

use super::catalogue::{
    ArgDefault, ArgSpec, DEFAULT_TIMEOUT, KindSpec, MAX_TIMEOUT,
};
use super::error::KindError;
use super::portable::Extras;
use super::raw::RawRow;

/// A row's args after the schema check: every given key is known, every
/// required key is there, every [`ArgDefault::Value`] is filled in.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Args {
    line: usize,
    values: BTreeMap<String, String>,
}

impl Args {
    /// The line of the row the args came from.
    #[must_use]
    pub fn line(&self) -> usize {
        self.line
    }

    /// The value of `key`, given or defaulted; `None` when absent, and
    /// for an [`ArgDefault::Forge`] key the row left out.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    /// The value of `key` as a whole number, `None` when absent.
    ///
    /// # Errors
    ///
    /// The value is not a whole number.
    pub fn number(&self, key: &str) -> Result<Option<u32>, KindError> {
        let Some(value) = self.get(key) else {
            return Ok(None);
        };
        value.parse().map(Some).map_err(|_| {
            KindError::new(
                self.line,
                format!("arg `{key}` is `{value}`, not a whole number"),
                format!("write `{key}=<number>`"),
            )
        })
    }

    /// The `timeout` in minutes, [`DEFAULT_TIMEOUT`] when absent.
    ///
    /// # Errors
    ///
    /// Not a whole number, or outside 1 to [`MAX_TIMEOUT`].
    pub fn timeout(&self) -> Result<u32, KindError> {
        let minutes = self.number("timeout")?.unwrap_or(DEFAULT_TIMEOUT);
        if (1..=MAX_TIMEOUT).contains(&minutes) {
            return Ok(minutes);
        }
        Err(KindError::new(
            self.line,
            format!("timeout `{minutes}` is not 1 to {MAX_TIMEOUT} minutes"),
            format!("write `timeout=<1..{MAX_TIMEOUT}>`"),
        ))
    }
}

/// The args of `row` checked against `kind`'s schema and the forge's
/// extra keys.
///
/// # Errors
///
/// An unknown or repeated key, or a missing required one (V31).
pub fn parse_args(
    kind: &KindSpec,
    row: &RawRow,
    extras: &Extras<'_>,
) -> Result<Args, KindError> {
    let known: Vec<&ArgSpec> = kind.schema.iter().chain(extras.keys).collect();
    let mut values = BTreeMap::new();
    for arg in &row.args {
        let spec = find(&known, row, &arg.key)?;
        if values.insert(key(spec), arg.value.clone()).is_some() {
            return Err(repeated(row.line, spec.key));
        }
    }
    for spec in &known {
        fill(&mut values, spec, row)?;
    }
    Ok(Args {
        line: row.line,
        values,
    })
}

/// The map key of `spec`.
fn key(spec: &ArgSpec) -> String {
    spec.key.to_owned()
}

/// The spec of `key`, or an unknown-key error listing the known ones.
fn find<'a>(
    known: &[&'a ArgSpec],
    row: &RawRow,
    key: &str,
) -> Result<&'a ArgSpec, KindError> {
    known
        .iter()
        .find(|s| s.key == key)
        .copied()
        .ok_or_else(|| unknown(known, row, key))
}

/// Fill `spec`'s default when the row left it out.
fn fill(
    values: &mut BTreeMap<String, String>,
    spec: &ArgSpec,
    row: &RawRow,
) -> Result<(), KindError> {
    if values.contains_key(spec.key) {
        return Ok(());
    }
    match spec.default {
        ArgDefault::Value(value) => {
            values.insert(key(spec), value.to_owned());
            Ok(())
        }
        ArgDefault::None if spec.required => Err(missing(row, spec.key)),
        ArgDefault::None | ArgDefault::Forge => Ok(()),
    }
}

fn unknown(known: &[&ArgSpec], row: &RawRow, key: &str) -> KindError {
    let what = format!("kind `{}` has no arg `{key}`", row.kind);
    if known.is_empty() {
        return KindError::new(row.line, what, "write `-` in args");
    }
    let keys: Vec<&str> = known.iter().map(|s| s.key).collect();
    let fix = format!("remove `{key}` or use one of: {}", keys.join(", "));
    KindError::new(row.line, what, fix)
}

fn repeated(line: usize, key: &str) -> KindError {
    KindError::new(
        line,
        format!("arg `{key}` is given twice"),
        format!("keep one `{key}=`"),
    )
}

fn missing(row: &RawRow, key: &str) -> KindError {
    KindError::new(
        row.line,
        format!("kind `{}` needs arg `{key}`", row.kind),
        format!("add `{key}=<value>` to args"),
    )
}

#[cfg(test)]
mod tests;
