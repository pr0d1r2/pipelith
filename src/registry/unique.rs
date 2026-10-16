//! A section's key column holds each value once (registry V51; actlith
//! `rows::unique`).

use super::document::Row;
use super::error::RegistryError;
use super::schema::SectionSpec;

/// The rows of `spec`'s section repeat no key; a section without a key
/// column passes.
///
/// # Errors
///
/// The second row with a key, naming the first one's line.
pub(crate) fn unique(
    _file: &str,
    _spec: &SectionSpec<'_>,
    _rows: &[Row],
) -> Result<(), RegistryError> {
    Err(RegistryError::default())
}

#[cfg(test)]
mod tests;
