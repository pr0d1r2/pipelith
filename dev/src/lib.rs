//! pipelith-dev: repository tooling: commit-message and changelog rules,
//! README badges, the third-party notices' crate table and the AGENTS.md
//! step table, each with a drift check. Never published (SPEC.md C, D18).
//! Ported from actlith-dev at `7ce69c6`. Re-exports only.

mod agents;
mod badges;
mod block;
mod changelog;
mod cli;
mod facts;
mod hooks;
mod io;
mod message;
mod notices;
mod readme;

pub use cli::run;
