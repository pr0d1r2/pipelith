//! The neutral catalogue (kind C, V34): every kind's name, arg schema,
//! default systems and secret purposes, as `const` data. Values come
//! from actlith `7ce69c6` (`src/gate/shape`, `src/build/kind`,
//! `src/bump`); a default that named a forge secret is
//! [`ArgDefault::Forge`]. The forge crate adds each kind's shape.

use super::row::Row;
use crate::system::{DEFAULT_SYSTEMS, System};

/// What a row that leaves an arg out gets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgDefault {
    /// Nothing: the arg is absent from the checked args.
    None,
    /// This value, the same on every forge.
    Value(&'static str),
    /// The forge fills it in (e.g. `token`: the forge's built-in job
    /// token); absent from the checked args (V34).
    Forge,
}

/// One arg a kind takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArgSpec {
    /// The key a row writes before `=`.
    pub key: &'static str,
    /// The row must give it.
    pub required: bool,
    /// What the row gets when it leaves the key out.
    pub default: ArgDefault,
}

/// Why a pipeline needs a secret; the forge maps it to a secret name
/// and its injection (V32).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Purpose {
    /// Push to the row's cachix cache; read-only use needs none.
    CachePush,
    /// Push a branch and open a change request.
    OpenChange,
}

/// One kind: what a `## workflows` row naming it may say.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KindSpec {
    /// The name a row writes in its `kind` cell.
    pub name: &'static str,
    /// Every arg the kind takes, in the order `docs/KINDS.md` lists them;
    /// any other key is refused.
    pub schema: &'static [ArgSpec],
    /// The systems a row's `-` means.
    pub default_systems: &'static [System],
    /// Every secret purpose the kind can need (V32).
    pub secrets: &'static [Purpose],
}

/// `timeout` (job minutes), which every kind takes (`docs/KINDS.md`).
pub const TIMEOUT: ArgSpec = ArgSpec {
    key: "timeout",
    required: false,
    default: ArgDefault::Value("30"),
};

/// Minutes a job runs when the row gives no `timeout`.
pub const DEFAULT_TIMEOUT: u32 = 30;

/// The most minutes a row's `timeout` may ask for.
pub const MAX_TIMEOUT: u32 = 360;

/// `token`: the secret that pushes the bump branch and opens the change
/// request. Its default is a token the forge crate names: the built-in
/// job token where that token can open a change request, otherwise a
/// dedicated secret.
const TOKEN: ArgSpec = ArgSpec {
    key: "token",
    required: false,
    default: ArgDefault::Forge,
};

/// The systems a bump kind runs on: one is enough to open a change.
const BUMP_SYSTEMS: [System; 1] = [System::X86_64Linux];

/// The hk gate on every supported system.
pub const GATE: KindSpec = KindSpec {
    name: "gate",
    schema: &[
        ArgSpec {
            key: "command",
            required: false,
            default: ArgDefault::Value(
                "nix develop --command hk check --all --check",
            ),
        },
        TIMEOUT,
    ],
    default_systems: &DEFAULT_SYSTEMS,
    secrets: &[Purpose::CachePush],
};

/// Build and cache a flake package, then prove the cache has it.
pub const BUILD: KindSpec = KindSpec {
    name: "build",
    schema: &[
        ArgSpec {
            key: "package",
            required: true,
            default: ArgDefault::None,
        },
        TIMEOUT,
    ],
    default_systems: &DEFAULT_SYSTEMS,
    secrets: &[Purpose::CachePush],
};

/// Follow `nixpkgs-lock` and propose the bump as a change request.
pub const UPDATE_PINS: KindSpec = KindSpec {
    name: "update-pins",
    schema: &[
        ArgSpec {
            key: "cron",
            required: false,
            default: ArgDefault::Value("50 6 * * *"),
        },
        ArgSpec {
            key: "build",
            required: false,
            default: ArgDefault::Value(".#default"),
        },
        TOKEN,
        TIMEOUT,
    ],
    default_systems: &BUMP_SYSTEMS,
    secrets: &[Purpose::OpenChange],
};

/// Poll an upstream release and propose the bump as a change request.
pub const UPSTREAM: KindSpec = KindSpec {
    name: "upstream",
    schema: &[
        ArgSpec {
            key: "script",
            required: true,
            default: ArgDefault::None,
        },
        ArgSpec {
            key: "cron",
            required: false,
            default: ArgDefault::Value("20 6 * * *"),
        },
        ArgSpec {
            key: "branch-prefix",
            required: false,
            default: ArgDefault::Value("upstream/"),
        },
        TOKEN,
        TIMEOUT,
    ],
    default_systems: &BUMP_SYSTEMS,
    secrets: &[Purpose::OpenChange],
};

/// Every neutral kind, in `docs/KINDS.md` order (V11).
pub const CATALOGUE: &[KindSpec] = &[GATE, BUILD, UPDATE_PINS, UPSTREAM];

/// The catalogue kind `name`, if there is one.
#[must_use]
pub fn lookup(name: &str) -> Option<&'static KindSpec> {
    CATALOGUE.iter().find(|k| k.name == name)
}

impl KindSpec {
    /// The secret purposes the pipeline built from `row` needs: a cache
    /// push only when the row names a cache (V32).
    #[must_use]
    pub fn purposes(&self, row: &Row) -> Vec<Purpose> {
        let cached = row.cache.is_some();
        self.secrets
            .iter()
            .copied()
            .filter(|&p| cached || p != Purpose::CachePush)
            .collect()
    }
}

#[cfg(test)]
mod tests;
