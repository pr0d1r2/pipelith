//! `src/registry`: the pipe-table registry grammar. Text and a caller
//! schema in, sections of rows of cells with line numbers out.
//!
//! The grammar knows no section's meaning: the caller's [`Schema`] names
//! the sections, their columns and keys, and the cell readers type each
//! cell. Composition only (src C): the logic lives in the submodules.
#![cfg_attr(
    not(test),
    allow(dead_code, reason = "RED: parse wires the helpers in GREEN")
)]

mod cells;
mod document;
mod error;
mod format;
mod parse;
mod schema;
mod unique;

pub use cells::{Arg, args, list, optional, required};
pub use document::{Cell, Document, Row, Section};
pub use error::RegistryError;
pub use format::SUPPORTED_FORMAT;
pub use parse::parse;
pub use schema::{Schema, SectionSpec};
