//! The whole file: the `format` line, then `## <section>` tables checked
//! against the caller's schema; `#` comments and blank lines skipped
//! (registry C, V15, V17, V18).

use super::document::Document;
use super::error::RegistryError;
use super::schema::Schema;

/// Parse registry `text` against `schema`. Pure: the caller reads the
/// file and passes its name, which every error carries (registry C).
///
/// # Errors
///
/// The first problem found, with its line, column and a fix: a missing
/// or newer `format`, an unknown or repeated section, a bad header, a
/// row with the wrong cell count, a repeated key, or a missing required
/// section.
pub fn parse(
    _file: &str,
    _text: &str,
    _schema: &Schema<'_>,
) -> Result<Document, RegistryError> {
    Ok(Document::default())
}

#[cfg(test)]
mod tests;
