//! The one system table (V20): every nix system pipelith knows, by its
//! nix name. No system name is spelled anywhere else in the library.

use std::fmt;

#[cfg(test)]
mod tests;

/// A nix system pipelith knows by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum System {
    /// `x86_64-linux`.
    X86_64Linux,
    /// `aarch64-linux`.
    Aarch64Linux,
    /// `aarch64-darwin`.
    Aarch64Darwin,
    /// `x86_64-darwin`: Intel macOS.
    X86_64Darwin,
}

/// Why a system name or system cannot be used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SystemError {
    /// The name is not a nix system in the table.
    Unknown(String),
    /// The forge has no default runner for the system and the
    /// repository names none in `## runners` (V22, src/runner V80).
    NoRunner(System),
}

/// One row: the system and its nix name.
type Row = (System, &'static str);

/// The table (V20).
const TABLE: [Row; 4] = [
    (System::X86_64Linux, "x86_64-linux"),
    (System::Aarch64Linux, "aarch64-linux"),
    (System::Aarch64Darwin, "aarch64-darwin"),
    (System::X86_64Darwin, "x86_64-darwin"),
];

/// Every system, in table order.
pub const ALL: [System; 4] = [
    System::X86_64Linux,
    System::Aarch64Linux,
    System::Aarch64Darwin,
    System::X86_64Darwin,
];

/// The systems a kind runs on when its row says `-`, and the set every
/// forge's runner table must cover (V22): every system (V21).
pub const DEFAULT_SYSTEMS: [System; 4] = ALL;

impl System {
    /// The nix name, exactly as nix spells it.
    #[must_use]
    pub fn name(self) -> &'static str {
        TABLE
            .into_iter()
            .find(|&(s, _)| s == self)
            .map_or("", |(_, n)| n)
    }
}

/// Parse a nix system name, exactly as nix spells it.
///
/// # Errors
///
/// [`SystemError::Unknown`] naming `name` when it is not in the table.
pub fn parse_system(name: &str) -> Result<System, SystemError> {
    TABLE
        .into_iter()
        .find(|&(_, n)| n == name)
        .map(|(s, _)| s)
        .ok_or_else(|| SystemError::Unknown(name.to_owned()))
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(name) => {
                write!(f, "unknown system `{name}`; known: ")?;
                let names: Vec<&str> = TABLE.iter().map(|r| r.1).collect();
                f.write_str(&names.join(", "))
            }
            Self::NoRunner(system) => write!(
                f,
                "system `{system}` has no default runner on this forge; \
                 name the repository's own runner in `## runners`: \
                 `{system}|<label>, <label>`"
            ),
        }
    }
}

impl std::error::Error for SystemError {}
