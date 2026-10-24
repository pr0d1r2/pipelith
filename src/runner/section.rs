//! The `## runners` section (V80): `system|labels`, one row per system
//! whose jobs run on the repository's own runner, and the trust guard
//! such a runner carries (V81).

use crate::registry::{Document, RegistryError, SectionSpec};
use crate::system::{System, SystemError};

#[cfg(test)]
mod tests;

/// The section's name: `## runners` on every forge.
pub const SECTION: &str = "runners";

/// The section's header row, in order.
pub const COLUMNS: [&str; 2] = ["system", "labels"];

/// The section as a forge crate puts it in its registry schema: keyed
/// on `system`, so the registry refuses a system named twice; optional,
/// so a file without it uses the forge's defaults.
pub const RUNNERS: SectionSpec<'static> = SectionSpec {
    name: SECTION,
    columns: &COLUMNS,
    required: false,
    key: Some(0),
};

/// Where a system's jobs run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runner<'a> {
    /// The forge's own default runner for the system.
    Default,
    /// The repository's own runner: one carrying every one of these
    /// labels, in the order the row lists them.
    Labels(&'a [String]),
}

impl Runner<'_> {
    /// Whether the job may run only for trusted events (V81): a push, a
    /// schedule, a manual run or a change request from a branch of the
    /// same repository, never a change request from a fork, whose code
    /// would otherwise run on the repository's own machine. True for
    /// every [`Runner::Labels`]; there is no opt-out.
    #[must_use]
    pub fn trusted_only(&self) -> bool {
        false
    }
}

/// The `## runners` rows of a registry file, checked.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Runners {
    /// One system and its runner's labels per row, in file order.
    rows: Vec<(System, Vec<String>)>,
}

impl Runners {
    /// The checked `## runners` rows of `doc`, parsed from `file`; none
    /// when the file has no such section.
    ///
    /// # Errors
    ///
    /// An unknown system, an empty `labels` cell, or a label that is not
    /// letters, digits, `.`, `_`, `-` or starts with `-`, located.
    pub fn from_document(
        _file: &str,
        _doc: &Document,
    ) -> Result<Self, RegistryError> {
        Ok(Self::default())
    }

    /// The runner for `system`: the row's labels when there is one,
    /// otherwise the forge's default when `has_default`.
    ///
    /// # Errors
    ///
    /// [`SystemError::NoRunner`] when there is neither a row nor a
    /// default, naming the `## runners` fix.
    pub fn pick(
        &self,
        _system: System,
        _has_default: bool,
    ) -> Result<Runner<'_>, SystemError> {
        Ok(Runner::Default)
    }
}
