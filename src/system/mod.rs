//! `src/system`: nix system names and the CI coverage set.
//!
//! The names and the coverage set are the same on every forge; which
//! runner runs a system is the forge crate's table (SPEC.md V2), or the
//! repository's own (`src/runner`). The table and its lookups live in `table` (V20); this file only
//! composes.

mod table;

pub use table::{ALL, DEFAULT_SYSTEMS, System, SystemError, parse_system};
