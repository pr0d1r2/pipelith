//! The commit-message rule (nix C, dev T18), as pure functions over the
//! message text.

/// The lines git keeps: up to the scissors line, `#` comment lines dropped.
#[must_use]
pub(crate) fn kept(text: &str) -> Vec<&str> {
    let _ = text;
    Vec::new()
}

/// The first line of the message git keeps.
#[must_use]
pub(crate) fn subject(text: &str) -> &str {
    let _ = text;
    ""
}

/// Subject `type(scope): claim` of at most 72 characters, and a body that
/// says why. Merge, revert, fixup and squash subjects are git's and pass.
///
/// # Errors
///
/// The rule broken, with the fix.
pub(crate) fn check(text: &str) -> Result<(), String> {
    let _ = text;
    Ok(())
}

#[cfg(test)]
mod tests;
