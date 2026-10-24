//! The `## runners` section (V80): `system|labels`, one row per system
//! whose jobs run on the repository's own runner, and the trust guard
//! such a runner carries (V81).

use crate::registry::{
    Document, RegistryError, Row as Cells, SectionSpec, list, required,
};
use crate::system::{System, SystemError, parse_system};

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
        matches!(self, Self::Labels(_))
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
        file: &str,
        doc: &Document,
    ) -> Result<Self, RegistryError> {
        let rows = doc.section(SECTION).iter();
        let rows = rows.map(|row| Ok((system(file, row)?, labels(file, row)?)));
        Ok(Self {
            rows: rows.collect::<Result<_, RegistryError>>()?,
        })
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
        system: System,
        has_default: bool,
    ) -> Result<Runner<'_>, SystemError> {
        match self.rows.iter().find(|(s, _)| *s == system) {
            Some((_, labels)) => Ok(Runner::Labels(labels)),
            None if has_default => Ok(Runner::Default),
            None => Err(SystemError::NoRunner(system)),
        }
    }
}

/// The row's `system` cell, a known nix system.
fn system(file: &str, row: &Cells) -> Result<System, RegistryError> {
    let name = required(file, row, 0, "system")?;
    parse_system(&name).map_err(|e| RegistryError {
        what: e.to_string(),
        fix: "write a nix system from the list".to_owned(),
        ..at(file, row, 0)
    })
}

/// The row's `labels` cell: at least one label, each a plain word.
fn labels(file: &str, row: &Cells) -> Result<Vec<String>, RegistryError> {
    required(file, row, 1, "labels")?;
    let labels = list(file, row, 1, "labels")?;
    if let Some(bad) = labels.iter().find(|l| !plain(l)) {
        return Err(RegistryError {
            what: format!("label `{bad}` is not a runner label"),
            fix: "use letters, digits, `.`, `_`, `-`; no leading `-`"
                .to_owned(),
            ..at(file, row, 1)
        });
    }
    Ok(labels)
}

/// Letters, digits and `._-`, not starting with `-`.
fn plain(label: &str) -> bool {
    let ok = |c: char| c.is_ascii_alphanumeric() || "._-".contains(c);
    label.chars().all(ok) && !label.starts_with('-')
}

/// An error pointing at cell `index` of `row`; the caller says what.
fn at(file: &str, row: &Cells, index: usize) -> RegistryError {
    RegistryError {
        file: file.to_owned(),
        line: row.line,
        column: row.cells.get(index).map_or(1, |c| c.column),
        ..RegistryError::default()
    }
}
