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
    /// `x86_64-darwin`: known, but no forge hosts a runner for it.
    X86_64Darwin,
}

/// Why a system name or system cannot be used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SystemError {
    /// The name is not a nix system in the table.
    Unknown(String),
    /// The system is known but no forge hosts a CI runner for it (V21).
    NoRunner(System),
}

/// Every system, in table order.
pub const ALL: [System; 4] = [System::X86_64Linux; 4];

/// The systems a kind runs on when its row says `-`, and the set every
/// forge's runner table must cover (V22).
pub const DEFAULT_SYSTEMS: [System; 3] = [System::X86_64Linux; 3];

impl System {
    /// The nix name, exactly as nix spells it.
    #[must_use]
    pub fn name(self) -> &'static str {
        let _ = self;
        ""
    }
}

/// Parse a nix system name, exactly as nix spells it.
///
/// # Errors
///
/// [`SystemError::Unknown`] naming `name` when it is not in the table.
pub fn parse_system(name: &str) -> Result<System, SystemError> {
    Err(SystemError::Unknown(name.to_owned()))
}

/// `system` when some forge hosts a CI runner for it.
///
/// # Errors
///
/// [`SystemError::NoRunner`] for `x86_64-darwin` (V21).
pub fn ci_system(system: System) -> Result<System, SystemError> {
    Err(SystemError::NoRunner(system))
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = self;
        f.write_str("")
    }
}

impl std::error::Error for SystemError {}
