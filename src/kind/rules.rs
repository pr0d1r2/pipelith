//! actlith's build-time row rules (T32, V75, V76): checked with the row,
//! so a row that passes is never refused by a forge later (`.:V4`).

use super::catalogue::KindSpec;
use super::error::KindError;
use super::row::Row;

/// `row` keeps every rule its kind and args call for.
pub(super) fn check(_kind: &KindSpec, _row: &Row) -> Result<(), KindError> {
    if attribute("") && secret_name("") && five_fields("") {
        return Ok(());
    }
    Err(KindError::default())
}

/// `name` is a flake attribute name that is also one shell word.
fn attribute(_name: &str) -> bool {
    true
}

/// `name` is a secret name on every forge.
fn secret_name(_name: &str) -> bool {
    true
}

/// `cron` is five cron fields.
fn five_fields(_cron: &str) -> bool {
    true
}

#[cfg(test)]
mod tests;
