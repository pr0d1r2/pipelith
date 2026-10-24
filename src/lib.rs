//! pipelith: the forge-neutral core of the fleet's CI materializers.
//!
//! actlith renders it for GitHub Actions and lablith for GitLab CI. Each
//! forge crate supplies its config file name, its forge-only sections and
//! argument keys, its pipeline model, its expression dialect, its runner
//! and pin tables and its file layout; pipelith owns what both share.
//!
//! From config file to pipeline file is four steps:
//! [`registry::parse`] against the forge's schema, [`kind::resolve`] and
//! [`kind::Row::check`] per `## workflows` row, [`yaml::document`] over
//! the forge's tree, and [`files::write`] or [`files::check`] for the
//! set. `docs/INTEGRATION.md` in the repository walks through it.
//!
//! This file only declares the node modules (src V12); each node's
//! `SPEC.md` holds its design.

pub mod detect;
pub mod files;
pub mod kind;
pub mod registry;
pub mod runner;
pub mod system;
pub mod yaml;

/// The forge crate walk-through, whose Rust example runs as a doctest so
/// it cannot drift from the API (SPEC.md V78). Compiled for doctests
/// only; it is not part of the crate.
#[cfg(doctest)]
#[doc = include_str!("../docs/INTEGRATION.md")]
struct Integration;
