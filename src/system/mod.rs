//! `src/system`: nix system names and the CI coverage set.
//!
//! The names and the refusal of `x86_64-darwin` are the same on every
//! forge; which runner runs a system is the forge crate's table (SPEC.md
//! V2). The table and its lookups live in `table` (V20); this file only
//! composes.

mod table;

pub use table::{
    ALL, DEFAULT_SYSTEMS, System, SystemError, ci_system, parse_system,
};
