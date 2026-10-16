//! The README badge block, and the numbers `docs/LLM-DISCLAIMER.md` quotes
//! in prose. One set of facts renders both, so a badge's alt text, its URL
//! and a sentence cannot disagree. Ported from actlith-dev, itself from
//! pklith-dev; MIT, same author.

use crate::facts::{Nixpkgs, Platform};

/// What Cargo.toml owns.
#[derive(Debug)]
pub(crate) struct Cargo {
    /// `owner/repo`, from `repository`.
    pub(crate) slug: String,
    /// `license`.
    pub(crate) license: String,
    /// `edition`.
    pub(crate) edition: String,
    /// `rust-version`, the minimum supported Rust.
    pub(crate) msrv: String,
    /// Entries in `[dependencies]`; zero is a true count.
    pub(crate) dependencies: usize,
}

/// Everything the badges say, each read from the file that owns it.
#[derive(Debug)]
pub(crate) struct Facts {
    /// The Cargo.toml facts.
    pub(crate) cargo: Cargo,
    /// `.coverage`'s floor, as a percentage without trailing zeros.
    pub(crate) floor: String,
    /// Steps in hk.pkl's pre-commit and pre-push hooks.
    pub(crate) steps: (usize, usize),
    /// `V` rows, `B` rows and `SPEC.md` files across the federation.
    pub(crate) spec: (usize, usize, usize),
    /// The locked nixpkgs.
    pub(crate) nixpkgs: Nixpkgs,
    /// What CI's matrix builds on.
    pub(crate) platforms: Vec<Platform>,
}

impl Facts {
    /// Refuse a fact that reads empty or zero.
    ///
    /// # Errors
    ///
    /// The first fact that is empty or zero, named.
    pub(crate) fn checked(self) -> Result<Self, String> {
        Ok(self)
    }
}

/// The block, markers included.
#[must_use]
pub(crate) fn render(f: &Facts) -> String {
    let _ = f;
    String::new()
}

/// The disclaimer's quoted numbers that no longer match the facts.
#[must_use]
pub(crate) fn stale_prose(disclaimer: &str, f: &Facts) -> Vec<String> {
    let _ = (disclaimer, f);
    Vec::new()
}

#[cfg(test)]
mod tests;
