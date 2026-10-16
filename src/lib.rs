//! pipelith: the forge-neutral core of the fleet's CI materializers.
//!
//! actlith renders it for GitHub Actions and lablith for GitLab CI. Each
//! forge crate supplies its config file name, its forge-only sections and
//! argument keys, its pipeline model, its expression dialect, its runner
//! and pin tables and its file layout; pipelith owns what both share.
//!
//! This file only declares the node modules (src V12); each node's
//! `SPEC.md` holds its design.

pub mod detect;
pub mod files;
pub mod kind;
pub mod registry;
pub mod system;
pub mod yaml;
