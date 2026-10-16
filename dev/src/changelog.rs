//! The changelog rule (SPEC.md V7, dev T18): a `feat`, `fix` or `perf`
//! commit that changes product code under `src/` adds an entry under
//! `## [Unreleased]` in CHANGELOG.md.

/// Whether a subject names a change consumers see: `feat`, `fix`, `perf`.
#[must_use]
pub(crate) fn behaviour(subject: &str) -> bool {
    let _ = subject;
    false
}

/// The staged paths under `src/` that are product code; specs and test
/// files are not.
#[must_use]
pub(crate) fn product(staged: &str) -> Vec<&str> {
    let _ = staged;
    Vec::new()
}

/// The `## [Unreleased]` section of a changelog, heading excluded.
#[must_use]
pub(crate) fn unreleased(changelog: &str) -> String {
    let _ = changelog;
    String::new()
}

/// The `[Unreleased]` sections the rule compares, and HEAD's subject.
#[derive(Debug, Default)]
pub(crate) struct Sections {
    /// As staged.
    pub(crate) staged: String,
    /// At HEAD.
    pub(crate) head: String,
    /// At HEAD's parent.
    pub(crate) parent: String,
    /// HEAD's subject.
    pub(crate) head_subject: String,
}

/// Pass when the change is not a behaviour change, touches no product
/// code, adds an entry, or amends a commit that brought its own entry.
///
/// # Errors
///
/// The rule broken, listing the product paths and the fix.
pub(crate) fn check(
    subject: &str,
    product: &[&str],
    s: &Sections,
) -> Result<(), String> {
    let _ = (subject, product, s);
    Ok(())
}

#[cfg(test)]
mod tests;
